//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends neofetch output.

use grammers_client::message::{InputMessage, Message};
use std::process::Command;

type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn knightcmd_neo(message: &Message) -> Result {
    let neofetch = Command::new("neofetch")
        .arg("--stdout")
        .output()
        .expect("Failed to execute command!");
    let text = String::from_utf8_lossy(&neofetch.stdout).to_string();
    let input_message = InputMessage::new().html(format!("<code>{}</code>", text.trim()));
    message.reply(input_message).await?;
    return Ok(());
}
