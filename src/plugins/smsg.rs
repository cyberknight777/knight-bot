//!
//! Copyright (C) 2023-2025 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends text and deletes original message.

use grammers_client::{
    Client,
    message::{InputMessage, Message},
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_smsg(client: Client, message: &Message, stext: String) -> Result {
    if stext.trim().is_empty() {
        message
            .reply(InputMessage::new().html("Send what? Give me <b>any text</b> to send!"))
            .await?;
        return Ok(());
    }

    let _ = message.delete().await;

    if let Some(id) = message.reply_to_message_id() {
        client
            .send_message(
                message.peer_ref().await.unwrap(),
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
