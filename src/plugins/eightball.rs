//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Rolls an eightball to say yes or no.

use crate::plugins;
use grammers_client::message::{InputMessage, Message};

type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn knightcmd_eightball(message: &Message) -> Result {
    let ball = plugins::random(2);
    let result = if ball == 0 {
        "Yes, it is the truth!"
    } else {
        "No, this is a prepostrous lie!"
    };
    if let Some(id) = message.reply_to_message_id() {
        message
            .respond(InputMessage::new().text(result).reply_to(Some(id)))
            .await?;
    } else {
        message.reply(result).await?;
    }
    return Ok(());
}
