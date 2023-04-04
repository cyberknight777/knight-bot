//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

use grammers_client::{Client, types::Message};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_start(client: Client, message: Message) -> Result {
    let msg = "Heya! Type /help to see what I can do!";
    client.send_message(message.chat(), msg).await?;
    return Ok(());
}
