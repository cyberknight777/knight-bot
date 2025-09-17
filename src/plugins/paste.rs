//!
//! Copyright (C) 2023-2025 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends a pastebin link of the replied message (or document) or the text given.

use grammers_client::{
    types::{InputMessage, Media, Message},
    Client,
};
use librustbin::Client as RbinClient;
use std::fs;
use tokio::fs as tokio_fs;
use tokio::io::AsyncReadExt;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

fn check_paste(url: &str) -> bool {
    !url.is_empty() || url != "This file is empty!" || url != "This file exceeds the file limit"
}

pub async fn knightcmd_paste(client: Client, message: Message, past: String) -> Result {
    const MAX_SIZE: i64 = 5 * 1024 * 1024;

    let msg = message
        .reply(InputMessage::html(format!("<b>Pasting content...</b>")))
        .await?;

    if let Some(reply) = client.get_reply_to_message(&message).await? {
        if let Some(ref media @ Media::Document(ref doc)) = reply.media() {
            if doc.size() > MAX_SIZE {
                msg.edit(InputMessage::html("<b>File too large (max 5MB)</b>"))
                    .await?;
                return Ok(());
            }

            let file_path = format!("/tmp/telegram_paste_{}", reply.id());

            client.download_media(&media, file_path.clone()).await?;

            let mut file = tokio_fs::File::open(&file_path).await?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).await?;

            let url = RbinClient::new("https://bin.cyberknight777.dev".to_string())
                .paste_highlight(contents)
                .unwrap()
                .trim()
                .to_string();

            if check_paste(&url) {
                msg.edit(format!("Link: {}", url)).await?;
            } else {
                msg.edit(InputMessage::html(format!("<b>Paste failed!</b>")))
                    .await?;
            }

            let _ = fs::remove_file(&file_path);
        } else if !reply.text().is_empty() {
            let url = RbinClient::new("https://bin.cyberknight777.dev".to_string())
                .paste_highlight(reply.text().to_string())
                .unwrap()
                .trim()
                .to_string();

            if check_paste(&url) {
                msg.edit(InputMessage::html(format!("Link: {}", url)))
                    .await?;
            } else {
                msg.edit(InputMessage::html("<b>Paste failed!</b>")).await?;
            }
        } else {
            msg.edit(InputMessage::html("<b>Paste failed!</b>")).await?;
        }
    } else if !past.is_empty() {
        let url = RbinClient::new("https://bin.cyberknight777.dev".to_string())
            .paste_highlight(past)
            .unwrap()
            .trim()
            .to_string();

        if check_paste(&url) {
            msg.edit(InputMessage::html(format!("Link: {}", url)))
                .await?;
        } else {
            msg.edit(InputMessage::html("<b>Paste failed!</b>")).await?;
        }
    } else {
        msg.edit(InputMessage::html(
            "Please reply to a <b>message</b> or reply with <b>/paste yourtext</b> to paste it!",
        ))
        .await?;
    }

    return Ok(());
}
