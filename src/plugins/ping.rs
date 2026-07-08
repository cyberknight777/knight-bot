//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Checks how fast I can respond.

use grammers_client::message::Message;
use std::time::Instant;

type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn knightcmd_ping(message: &Message) -> Result {
    let start = Instant::now();
    let msg = message.reply("Pinging........").await?;
    msg.edit(format!("Pong! {}ms", start.elapsed().as_millis()))
        .await?;
    return Ok(());
}
