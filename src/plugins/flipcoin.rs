//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

use crate::plugins;
use grammers_client::types::Message;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_flipcoin(message: Message) -> Result {
    let coin = plugins::random(2);
    let result = if coin == 0 { "Heads!" } else { "Tails!" };
    message.reply(result).await?;
    return Ok(());
}
