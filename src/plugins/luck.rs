//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Says your lucky number.

use crate::plugins;
use grammers_client::message::{InputMessage, Message};

type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn knightcmd_luck(message: &Message) -> Result {
    let random_number = plugins::random(101); // modulo 101 to get a number between 0 to 100
    if let Some(id) = message.reply_to_message_id() {
        message
            .respond(
                InputMessage::new()
                    .html(format!(
                        "Your lucky number is: <code>{}</code>",
                        random_number
                    ))
                    .reply_to(Some(id)),
            )
            .await?;
    } else {
        message
            .reply(InputMessage::new().html(format!(
                "Your lucky number is: <code>{}</code>",
                random_number
            )))
            .await?;
    }
    return Ok(());
}
