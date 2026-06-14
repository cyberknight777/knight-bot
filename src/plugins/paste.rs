//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends a pastebin link of the replied message (or document) or the text given.

use grammers_client::{
    Client,
    media::Media,
    message::{InputMessage, Message},
};
use librustbin::{Client as RbinClient, PasteOptions};
use std::fs;
use tokio::fs as tokio_fs;
use tokio::io::AsyncReadExt;

type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

fn check_paste(url: &str) -> bool {
    !url.is_empty() && url != "This file is empty!" && url != "This file exceeds the file limit"
}

async fn fail_edit(msg: &Message) -> Result {
    msg.edit(InputMessage::new().html("<b>Paste failed!</b>"))
        .await?;
    return Ok(());
}

async fn paste_edit(msg: &Message, content: String, filename: Option<String>) -> Result {
    let rbin = RbinClient::new("https://bin.cyberknight777.dev".to_string());

    let options = PasteOptions {
        filename,
        ..Default::default()
    };

    match rbin.paste(&content, &options).await {
        Ok(url_raw) => {
            let url = url_raw.trim().to_string();
            if check_paste(&url) {
                msg.edit(
                    InputMessage::new()
                        .html(format!("Link: {}", url))
                        .link_preview(true),
                )
                .await?;
            } else {
                fail_edit(&msg).await?;
            }
        }
        Err(_) => {
            fail_edit(&msg).await?;
        }
    }
    return Ok(());
}

pub async fn knightcmd_paste(client: Client, message: &Message, past: String) -> Result {
    const MAX_SIZE: usize = 5 * 1024 * 1024;

    let msg = message
        .reply(InputMessage::new().html("<b>Pasting content...</b>"))
        .await?;

    match message.get_reply().await? {
        Some(reply) => match reply.media() {
            Some(ref media @ Media::Document(ref doc)) => {
                if doc.size().unwrap_or(0) > MAX_SIZE {
                    msg.edit(InputMessage::new().html(format!(
                        "<b>File too large (max {}MB)</b>",
                        MAX_SIZE / (1024 * 1024)
                    )))
                    .await?;
                    return Ok(());
                }

                let file_path = format!("/tmp/telegram_paste_{}", reply.id());

                client.download_media(media, file_path.clone()).await?;

                let mut file = tokio_fs::File::open(&file_path).await?;
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes).await?;

                let contents = String::from_utf8_lossy(&bytes).to_string();

                let filename = doc
                    .name()
                    .filter(|name| !name.is_empty())
                    .map(|name| name.to_string());

                paste_edit(&msg, contents, filename).await?;

                let _ = fs::remove_file(&file_path);
            }
            _ => {
                if !reply.text().is_empty() {
                    paste_edit(&msg, reply.text().to_string(), None).await?;
                } else {
                    fail_edit(&msg).await?;
                }
            }
        },
        _ => {
            if !past.is_empty() {
                paste_edit(&msg, past, None).await?;
            } else {
                msg.edit(InputMessage::new().html(
            "Please reply to a <b>message</b> or reply with <b>/paste yourtext</b> to paste it!",
        ))
        .await?;
            }
        }
    }

    return Ok(());
}
