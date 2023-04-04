//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

use grammers_client::{Client, types::Message};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_msg(client: Client, message: Message, text: String) -> Result {
    client.send_message(message.chat(), text).await?;
    return Ok(());
}
