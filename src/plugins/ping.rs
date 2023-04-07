//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

use std::time::SystemTime;

use grammers_client::types::Message;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_ping(message: Message) -> Result {
    let start = SystemTime::now();
    let msg = message.reply("Pinging........").await?;
    let end = SystemTime::now();
    let ping = end.duration_since(start).unwrap().as_millis();
    msg.edit(format!("Pong! {}ms", ping)).await?;
    return Ok(());
}
