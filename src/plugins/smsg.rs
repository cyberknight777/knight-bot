//!
//! Copyright (C) 2023-2025 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends text and deletes original message.

use grammers_client::{
    types::{InputMessage, Message},
    Client,
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_smsg(client: Client, message: Message, stext: String) -> Result {
    if stext.trim().is_empty() {
        message
            .reply(InputMessage::html(
                "Send what? Give me <b>any text</b> to send!",
            ))
            .await?;
        return Ok(());
    }

    let _ = message.delete().await;

    if let Some(id) = message.reply_to_message_id() {
        client
            .send_message(
                message.chat(),
                InputMessage::markdown(stext.trim().replace(r#"\n"#, "  \n")).reply_to(Some(id)),
            )
            .await?;
    } else {
        message
            .reply(InputMessage::markdown(
                stext.trim().replace(r#"\n"#, "  \n"),
            ))
            .await?;
    }
    return Ok(());
}
