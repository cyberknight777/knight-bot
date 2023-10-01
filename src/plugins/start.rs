//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Checks if I'm alive

use grammers_client::types::Message;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_start(message: Message) -> Result {
    let msg = "Heya! Type /help to see what I can do!";
    message.reply(msg).await?;
    return Ok(());
}
