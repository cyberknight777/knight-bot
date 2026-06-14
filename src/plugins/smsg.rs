//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends text and deletes original message.

use grammers_client::message::{InputMessage, Message};

type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn knightcmd_smsg(message: &Message, stext: String) -> Result {
    if stext.trim().is_empty() {
        message
            .reply(InputMessage::new().html("Send what? Give me <b>any text</b> to send!"))
            .await?;
        return Ok(());
    }

    let _ = message.delete().await;

    if let Some(id) = message.reply_to_message_id() {
        message
            .respond(
                InputMessage::new()
                    .markdown(stext.trim().replace(r#"\n"#, "  \n"))
                    .reply_to(Some(id)),
            )
            .await?;
    } else {
        message
            .reply(InputMessage::new().markdown(stext.trim().replace(r#"\n"#, "  \n")))
            .await?;
    }
    return Ok(());
}
