//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

use grammers_client::{
    Client,
    types::{InputMessage, Message}};
use std::process::Command;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_neo(client: Client, message: Message) -> Result {
    let neofetch = Command::new("osfetch-rs").arg("--stdout").output().expect("Failed to execute command!");
    let text = String::from_utf8_lossy(&neofetch.stdout).to_string();
    let input_message = InputMessage::html(format!("<code>{}</code>", text.trim()));
    client.send_message(message.chat(), input_message).await?;
    return Ok(());
}
