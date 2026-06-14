//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends text.

use grammers_client::{
    Client,
    message::{InputMessage, Message},
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_msg(client: Client, message: &Message, text: String) -> Result {
    if text.trim().is_empty() {
        message
            .reply(InputMessage::new().html("Send what? Give me <b>any text</b> to send!"))
            .await?;
        return Ok(());
    }
    if let Some(id) = message.reply_to_message_id() {
        client
            .send_message(
                message.peer_ref().await.unwrap(),
                InputMessage::new()
                    .markdown(text.trim().replace(r#"\n"#, "  \n"))
                    .reply_to(Some(id)),
            )
            .await?;
    } else {
        message
            .reply(InputMessage::new().markdown(text.trim().replace(r#"\n"#, "  \n")))
            .await?;
    }
    return Ok(());
}
