//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Checks how fast I can respond

use grammers_client::types::Message;
use std::time::SystemTime;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_ping(message: Message) -> Result {
    let start = SystemTime::now();
    let msg = message.reply("Pinging........").await?;
    let end = SystemTime::now();
    let ping = end.duration_since(start).unwrap().as_millis();
    msg.edit(format!("Pong! {}ms", ping)).await?;
    return Ok(());
}
