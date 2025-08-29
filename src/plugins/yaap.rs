//!
//! Copyright (C) 2023-2025 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Gets the latest YAAP release according to the device.

use crate::plugins;
use grammers_client::{
    button, reply_markup,
    types::{InputMessage, Message},
    Client,
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_yaap(client: Client, message: Message, device: String) -> Result {
    if device.trim().is_empty() {
        message
            .reply(InputMessage::html("Provide a device <b>codename</b>!"))
            .await?;
        return Ok(());
    }

    let branch = format!(
        "https://raw.githubusercontent.com/YAAP/device-info/master/{}/{}.json",
        device, device
    );

    let branch_resp = plugins::req::make_request(branch.to_string()).await;

    let gapps_branch;

    let vanilla_branch;

    match branch_resp {
        Some(branch_resp) => {
            gapps_branch = branch_resp["ota-branch"]
                .to_string()
                .trim_matches('"')
                .to_string();
            vanilla_branch = branch_resp["ota-branch-vanilla"]
                .to_string()
                .trim_matches('"')
                .to_string();
        }
        None => {
            message
                .reply(InputMessage::html(
                    "Failed to get YAAP release information! (OTA Branch)",
                ))
                .await?;
            return Ok(());
        }
    }

    let gapps = format!(
        "https://raw.githubusercontent.com/YAAP/ota-info/{}/{}/{}.json",
        gapps_branch, device, device
    );
    let gapps_resp = plugins::req::make_request(gapps.to_string()).await;

    let vanilla = format!(
        "https://raw.githubusercontent.com/YAAP/ota-info/{}/{}/{}.json",
        vanilla_branch, device, device
    );
    let vanilla_resp = plugins::req::make_request(vanilla.to_string()).await;

    let gapps_link;

    let vanilla_link;

    match gapps_resp {
        Some(gapps_resp) => {
            gapps_link = gapps_resp["response"]
                .as_array()
                .and_then(|arr| arr.first())
                .and_then(|obj| obj.get("filename"))
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown filename")
                .to_string();
        }
        None => {
            message
                .reply(InputMessage::html(
                    "Failed to get YAAP release information! (Gapps)",
                ))
                .await?;
            return Ok(());
        }
    }
    match vanilla_resp {
        Some(vanilla_resp) => {
            vanilla_link = vanilla_resp["response"]
                .as_array()
                .and_then(|arr| arr.first())
                .and_then(|obj| obj.get("filename"))
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown filename")
                .to_string();
        }
        None => {
            message
                .reply(InputMessage::html(
                    "Failed to get YAAP release information! (Vanilla)",
                ))
                .await?;
            return Ok(());
        }
    }

    if let Some(id) = message.reply_to_message_id() {
        client
            .send_message(
                message.chat(),
                InputMessage::html(format!("<b>Latest YAAP Releases for {}</b>:", device))
                    .reply_to(Some(id))
                    .reply_markup(&reply_markup::inline(vec![
                        vec![button::url(
                            "Gapps",
                            format!(
                                "https://mirror.codebucket.de/yaap/{}/{}",
                                device,
                                gapps_link.to_string().trim_matches('"').to_string()
                            ),
                        )],
                        vec![button::url(
                            "Vanilla",
                            format!(
                                "https://mirror.codebucket.de/yaap/{}/vanilla/{}",
                                device,
                                vanilla_link.to_string().trim_matches('"').to_string()
                            ),
                        )],
                    ])),
            )
            .await?;
    } else {
        message
            .reply(
                InputMessage::html(format!("<b>Latest YAAP Releases for {}</b>:", device))
                    .reply_markup(&reply_markup::inline(vec![
                        vec![button::url(
                            "Gapps",
                            format!(
                                "https://mirror.codebucket.de/yaap/{}/{}",
                                device,
                                gapps_link.to_string().trim_matches('"').to_string()
                            ),
                        )],
                        vec![button::url(
                            "Vanilla",
                            format!(
                                "https://mirror.codebucket.de/yaap/{}/vanilla/{}",
                                device,
                                vanilla_link.to_string().trim_matches('"').to_string()
                            ),
                        )],
                    ])),
            )
            .await?;
    }
    return Ok(());
}
