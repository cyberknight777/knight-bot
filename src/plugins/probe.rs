//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Probe through a provided MediaTek partition to view the contents.

use crate::plugins::dlp;
use grammers_client::{
    Client,
    media::Media,
    message::{InputMessage, Message},
};
use hacc::{Da, Image, Preloader, TryRead, gfh::Gfh};
use regex::Regex;
use std::path::Path;
use std::time::Duration;
use tokio::time;

type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

const DOWNLOAD_USAGE: &str = "Reply to a <b>valid MediaTek partition</b>!";
const DOWNLOAD_FAILED: &str = "<b>Download failed!</b>";
const DOWNLOAD_STARTED: &str = "<b>Downloading file...</b>";

// ARM32 `bx r0` (0xe12fff10): load addr (u32) is 8 bytes after it.
const ARM32_BX_R0: [u8; 4] = [0x10, 0xff, 0x2f, 0xe1];
// DTB on-disk magic (0xd00dfeed, big-endian).
const DTB_MAGIC: [u8; 4] = [0xd0, 0x0d, 0xfe, 0xed];
// Max size of image being parsed.
const MAX_SIZE: usize = 256 * 1024 * 1024;

fn extract_load_addr(content: &[u8]) -> Option<u64> {
    for window in content.windows(12).step_by(4) {
        if window[0..4] == ARM32_BX_R0 {
            let lo = u32::from_le_bytes(window[8..12].try_into().unwrap()) as u64;
            return Some(lo);
        }
    }
    for chunk in content.windows(8).step_by(4) {
        let lo = u32::from_le_bytes(chunk[0..4].try_into().unwrap()) as u64;
        let hi = u32::from_le_bytes(chunk[4..8].try_into().unwrap()) as u64;
        if hi == 0xffff_0000 && (0x4000_0000..0xa000_0000).contains(&lo) {
            return Some((hi << 32) | lo);
        }
    }
    None
}

fn extract_datetime(s: &str) -> Option<String> {
    let re = Regex::new(r"\d{4}[-/]\d{2}[-/]\d{2}[ .]\d{2}:\d{2}(:\d{2})?").unwrap();

    re.find(s)
        .map(|m| m.as_str().replace('.', " ").replace('/', "-"))
}

fn parse_img(data: &Vec<u8>) -> String {
    let image = Image::new(&data);
    let mut msg = String::from("<b>Info</b>\n");
    let is_lk = image.has_partition("lk");
    let has_bl2_ext = image.has_partition("bl2_ext");
    let size = image.data.len();
    let len = image.partitions().count();

    if image.partitions().next().is_some() {
        msg.push_str(&format!("  Partitions: <code>{len}</code>\n"));
        msg.push_str(&format!("  Size: <code>{size}</code> bytes\n"));

        if is_lk {
            let version = if has_bl2_ext {
                "2 (ARM64)"
            } else {
                "1 (ARM32)"
            };
            msg.push_str(&format!("  Version: <code>{version}</code>\n"));
        }

        msg.push_str("\n<b>Partitions</b>\n");

        for part in image.partitions() {
            let name = part.header.name();
            let dsize = part.header.data_size();
            let mode = part.header.mode();
            let addr = if part.content.starts_with(&DTB_MAGIC) {
                part.header.addr()
            } else {
                extract_load_addr(part.content).unwrap_or_else(|| part.header.addr())
            };
            let hdr = if part.header.is_extended() {
                "Extended"
            } else {
                "Legacy"
            };

            msg.push_str(&format!("  <code>{name}</code>\n    Size: <code>{dsize}</code> bytes\n    Mode: <code>0x{:x}</code>\n    Addr: <code>0x{:x}</code>\n    Header: <code>{hdr}</code>", mode, addr));

            let certs: Vec<_> = image
                .get_part_certs(name)
                .map(|c| c.header.name().to_string())
                .collect();

            if !certs.is_empty() {
                msg.push_str("\n    Certs: <code>");
                msg.push_str(&certs.join(", "));
                msg.push_str("</code>\n");
            }

            msg.push('\n');
        }
        return msg;
    } else {
        msg.clear();
        return msg;
    }
}

fn parse_pl(data: &Vec<u8>) -> String {
    let mut msg = String::from("<b>GFH File Info</b>\n");
    match Preloader::try_read(&data) {
        Ok(preloader) => {
            let info = preloader.gfh().file_info();
            let gfhs: Vec<_> = preloader.gfh().gfhs().collect();
            let flash_dev = match info.flash_dev() {
                hacc::gfh::GfhFlashDev::None => "NONE",
                hacc::gfh::GfhFlashDev::Nor => "NOR",
                hacc::gfh::GfhFlashDev::NandSeq => "NAND_SEQ",
                hacc::gfh::GfhFlashDev::NandTtbl => "NAND_TTBL",
                hacc::gfh::GfhFlashDev::NandFdm50 => "NAND_FDM50",
                hacc::gfh::GfhFlashDev::EmmcBoot => "EMMC_BOOT",
                hacc::gfh::GfhFlashDev::EmmcData => "EMMC_DATA",
                hacc::gfh::GfhFlashDev::Sf => "SF",
                hacc::gfh::GfhFlashDev::Xboot => "XBOOT",
                hacc::gfh::GfhFlashDev::SpiNand => "SPI_NAND",
                hacc::gfh::GfhFlashDev::Ufs => "UFS",
                hacc::gfh::GfhFlashDev::Combo => "COMBO",
            };

            msg.push_str(&format!(
		"  File Type: <code>{:#06x}</code>\n  Flash Dev: <code>{}</code>\n  Sig Type: <code>{:?}</code>\n  Load Addr: <code>{:#010x}</code>\n",
		info.file_type() as u16,
		flash_dev,
		info.sig_type(),
		info.load_addr(),
	    ));
            msg.push_str(&format!(
		"  Total: <code>{}</code> bytes\n  Content: <code>{}</code> bytes\n  Signature: <code>{}</code> bytes\n",
		info.file_len(),
		info.content_len(),
		info.sig_len(),
	    ));
            msg.push_str(&format!(
                "  Jump Off: <code>{:#010x}</code>\n  Content Base: <code>{:#010x}</code>\n",
                info.jump_offset(),
                info.load_addr() + info.content_offset()
            ));
            msg.push_str(&format!("\n<b>GFH Sections</b> ({} total)\n", gfhs.len()));

            for gfh in gfhs {
                match gfh {
                    hacc::gfh::GfhKind::FileInfo(info) => {
                        let header = info.header();
                        msg.push_str(&format!(
                            "  <code>FILE_INFO</code> (v{}, {} bytes)\n",
                            header.version(),
                            header.size()
                        ));
                    }
                    hacc::gfh::GfhKind::BlInfo(bl) => {
                        let header = bl.header();

                        msg.push_str(&format!(
                            "  <code>BL_INFO</code> (v{}, {} bytes)\n",
                            header.version(),
                            header.size()
                        ));
                    }
                    hacc::gfh::GfhKind::AntiClone(anti) => {
                        let header = anti.header();

                        msg.push_str(&format!(
                            "  <code>ANTI_CLONE</code> (v{}, {} bytes)\n",
                            header.version(),
                            header.size()
                        ));

                        msg.push_str(&format!(
			    "    AC Offset: <code>{:#010x}</code>\n    AC Length: <code>{}</code>\n",
			    anti.ac_offset(),
			    anti.ac_length()
			));
                    }
                    hacc::gfh::GfhKind::BromSecCfg(sec) => {
                        let header = sec.header();
                        let jtag = if sec.jtag_enabled() { "yes" } else { "no" };
                        let dbg = if sec.debug_enabled() { "yes" } else { "no" };

                        msg.push_str(&format!(
                            "  <code>BROM_SEC_CFG</code> (v{}, {} bytes)\n",
                            header.version(),
                            header.size()
                        ));
                        msg.push_str(&format!(
                            "    JTAG: <code>{}</code>\n    Debug: <code>{}</code>\n",
                            jtag, dbg,
                        ));
                    }
                    hacc::gfh::GfhKind::BromCfgV3(cfg) => {
                        let header = cfg.header();
                        let uart1 = if cfg.get_uart1_log_disabled() {
                            "no"
                        } else {
                            "yes"
                        };
                        let usbau = if cfg.get_usbdl_auto_detect_disabled() {
                            "disabled"
                        } else {
                            "enabled"
                        };
                        let bootarch = if cfg.get_jump_bl_aarch64_enabled() {
                            "yes"
                        } else {
                            "no"
                        };

                        msg.push_str(&format!(
                            "  <code>BROM_CFG</code> (v{}, {} bytes)\n",
                            header.version(),
                            header.size()
                        ));
                        msg.push_str(&format!(
			    "    UART1 Log Dis: <code>{}</code>\n    USB Auto Detect: <code>{}</code>\n    Boot AArch64: <code>{}</code>\n",
			    uart1,
			    usbau,
			    bootarch
			));
                    }
                    _ => {
                        let header = gfh.header();

                        msg.push_str(&format!(
                            "  <code>{:#06x}</code> (v{}, {} bytes)\n",
                            header.gfh_type() as u16,
                            header.version(),
                            header.size()
                        ));
                    }
                }
            }

            return msg;
        }
        Err(_) => {
            msg.clear();
            return msg;
        }
    }
}

fn parse_da(data: &Vec<u8>) -> String {
    let mut msg = String::from("<b>Info</b>\n");
    match Da::try_read(&data) {
        Ok(da) => {
            let entry = match da.entry(0) {
                Some(x) => x,
                None => {
                    msg.clear();
                    return msg;
                }
            };
            let version = entry.version();
            let entries = da.header().da_count();
            let hw_code = entry.hw_code();
            let hw_sub_code = entry.hw_sub_code();
            let date = extract_datetime(da.header().desc()).unwrap_or("Unknown".to_string());
            msg.push_str(&format!(
                "  Type: <code>{:?}</code>\n  Entries: <code>{}</code>\n  Built: <code>{}</code>\n",
                version, entries, date
            ));

            msg.push_str(&format!(
                "  HW Code: <code>0x{:x}</code>\n  HW Sub: <code>0x{:x}</code>\n\n",
                hw_code, hw_sub_code
            ));

            msg.push_str("<b>Regions</b>\n");

            for (i, region) in entry.regions().iter().enumerate() {
                msg.push_str(&format!(
		    "  Region <code>{}</code>\n    Offset: <code>0x{:08x}</code>\n    Length: <code>0x{:08x}</code>\n    Addr: <code>0x{:08x}</code>\n    Sig: <code>{}</code> bytes\n\n",
		    i,
		    region.offset(),
		    region.length(),
		    region.addr(),
		    region.sig_len()
		));
            }
            return msg;
        }
        Err(_) => {
            msg.clear();
            return msg;
        }
    }
}

async fn download_reply_media(client: Client, message: &Message, media: Media) -> Result {
    let status = message
        .reply(InputMessage::new().html(DOWNLOAD_STARTED))
        .await?;
    let filename = dlp::filename_from_document(&media);
    let ext = Path::new(&filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if !matches!(ext.as_str(), "bin" | "img") {
        status
            .edit(InputMessage::new().html("<b>Unsupported file type!</b>"))
            .await?;
        return Ok(());
    }
    let data = match async {
        let mut data = Vec::new();
        let mut download = client.iter_download(&media);

        while let Some(chunk) = download.next().await? {
            data.extend(chunk);

            if data.len() > MAX_SIZE {
                return Err(grammers_client::InvocationError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "file too large",
                )));
            }
        }

        Ok::<Vec<u8>, grammers_client::InvocationError>(data)
    }
    .await
    {
        Ok(data) => data,
        Err(e) => {
            let msg = match e {
                grammers_client::InvocationError::Io(err)
                    if err.to_string() == "file too large" =>
                {
                    "<b>File too large!</b>"
                }
                _ => DOWNLOAD_FAILED,
            };

            status.edit(InputMessage::new().html(msg)).await?;
            return Ok(());
        }
    };

    // First we try to parse the data as a valid MediaTek image.
    let mut result = parse_img(&data);

    if !result.is_empty() {
        status
            .edit(InputMessage::new().html("<b>Parsing as MediaTek image...</b>"))
            .await?;
        time::sleep(Duration::from_secs(1)).await;
        status.edit(InputMessage::new().html(result)).await?;
        return Ok(());
    }

    // If it isn't a valid MediaTek image, then we try to parse it as a valid MediaTek Preloader.
    result = parse_pl(&data);

    if !result.is_empty() {
        status
            .edit(InputMessage::new().html("<b>Parsing as MediaTek Preloader...</b>"))
            .await?;
        time::sleep(Duration::from_secs(1)).await;
        status.edit(InputMessage::new().html(result)).await?;
        return Ok(());
    }

    // If it isn't a valid MediaTek Preloader either, then we finally parse it as a valid MediaTek Download Agent.
    result = parse_da(&data);

    if !result.is_empty() {
        status
            .edit(InputMessage::new().html("<b>Parsing as MediaTek Download Agent (DA)...</b>"))
            .await?;
        time::sleep(Duration::from_secs(1)).await;
        status.edit(InputMessage::new().html(result)).await?;
        return Ok(());
    }

    status
        .edit(InputMessage::new().html("<b>Failed to parse as a valid MediaTek partition!</b>"))
        .await?;

    return Ok(());
}

pub async fn knightcmd_probe(client: Client, message: &Message) -> Result {
    if let Some(reply) = message.get_reply().await? {
        if let Some(media) = reply.media() {
            download_reply_media(client, message, media).await?;
        } else {
            message
                .reply(InputMessage::new().html(DOWNLOAD_USAGE))
                .await?;
        }
    } else {
        message
            .reply(InputMessage::new().html(DOWNLOAD_USAGE))
            .await?;
    }
    return Ok(());
}
