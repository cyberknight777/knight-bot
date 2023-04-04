//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

use grammers_client::{Client, types::Message};
use getrandom::getrandom;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_flipcoin(_client: Client, message: Message) -> Result {
    let mut buffer = [0; 1];
    getrandom(&mut buffer).expect("Failed to generate random number");
    let coin = buffer[0] % 2;
    let result = if coin == 0 { "Heads!" } else { "Tails!" };
    message.reply(result).await?;
    return Ok(());
}
