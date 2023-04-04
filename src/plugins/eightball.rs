//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

use grammers_client::{Client, types::Message};
use getrandom::getrandom;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_eightball(_client: Client, message: Message) -> Result {
    let mut buffer = [0; 1];
    getrandom(&mut buffer).expect("Failed to generate random number");
    let ball = buffer[0] % 2;
    let result = if ball == 0 { "Yes, it is the truth!" } else { "No, this is a prepostrous lie!" };
    message.reply(result).await?;

    return Ok(());
}
