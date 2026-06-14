//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Gets UserID and ChatID.

use grammers_client::message::{InputMessage, Message};

type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn knightcmd_uid(message: &Message) -> Result {
    if let Some(id) = message.reply_to_message_id() {
        if let Some(reply_to_msg) = message.get_reply().await? {
            if let Some(sender) = reply_to_msg.sender() {
                message
                    .respond(
                        InputMessage::new()
                            .html(format!(
                                "Your ID: <code>{}</code>
ChatID: <code>-100{}</code>
{}'s ID: <code>{}</code>",
                                message.sender().and_then(|s| s.id().bare_id()).unwrap_or(0),
                                message.peer_id().bare_id().unwrap_or(0),
                                sender.name().unwrap_or(""),
                                sender.id().bare_id().unwrap_or(0)
                            ))
                            .reply_to(Some(id)),
                    )
                    .await?;
            }
        }
    } else {
        message
            .reply(InputMessage::new().html(format!(
                "Your ID: <code>{}</code>
ChatID: <code>-100{}</code>",
                message.sender().and_then(|s| s.id().bare_id()).unwrap_or(0),
                message.peer_id().bare_id().unwrap_or(0)
            )))
            .await?;
    }
    return Ok(());
}
