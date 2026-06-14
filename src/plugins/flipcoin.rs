//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Flips a coin to say heads or tails.

use crate::plugins;
use grammers_client::message::{InputMessage, Message};

type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn knightcmd_flipcoin(message: &Message) -> Result {
    let coin = plugins::random(2);
    let result = if coin == 0 { "Heads!" } else { "Tails!" };
    if let Some(id) = message.reply_to_message_id() {
        message
            .respond(InputMessage::new().text(result).reply_to(Some(id)))
            .await?;
    } else {
        message.reply(result).await?;
    }
    return Ok(());
}
