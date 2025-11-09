//!
//! Copyright (C) 2023-2025 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends a shortlink of the replied link or the link given.

use grammers_client::{
    types::{InputMessage, update::Message},
    Client,
};
use librustbin::Client as RbinClient;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

fn check_paste(url: &str) -> bool {
    !url.is_empty() && url != "This file is empty!" && url != "relative URL without a base"
}

pub async fn knightcmd_lpaste(client: Client, message: Message, link: String) -> Result {
    let msg = message
        .reply(InputMessage::html("<b>Pasting link...</b>"))
        .await?;

    let text_to_paste = if let Some(reply) = client.get_reply_to_message(&message).await? {
        if !reply.text().is_empty() {
            Some(reply.text().to_string())
        } else {
            None
        }
    } else if !link.is_empty() {
        Some(link)
    } else {
        None
    };

    if let Some(text) = text_to_paste {
        let rbin = RbinClient::new("https://bin.cyberknight777.dev".to_string());

        match rbin.paste_short(text) {
            Ok(url_raw) => {
                let url = url_raw.trim().to_string();
                if check_paste(&url) {
                    msg.edit(InputMessage::html(format!("Link: {}", url)))
                        .await?;
                } else {
                    msg.edit(InputMessage::html("<b>Paste failed!</b>")).await?;
                }
            }
            Err(_) => {
                msg.edit(InputMessage::html("<b>Paste failed!</b>")).await?;
            }
        }
    } else {
        msg.edit(InputMessage::html(
            "Please reply to a <b>link</b> or reply with <b>/lpaste https://link.com</b> to shortlink it!",
        )).await?;
    }
    return Ok(());
}
