//!
//! Copyright (C) 2023-2025 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

use grammers_client::types::{InputMessage, update::Message};
use std::process::Command;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_sh(message: Message, kcmd: String) -> Result {
    if kcmd.trim().is_empty() {
        message.reply(InputMessage::html("Dude! With all due respect that you're my maker and all, give me a <b>proper command</b> to run!")).await?;
        return Ok(());
    }
    let command = Command::new("bash")
        .arg("-c")
        .arg(kcmd)
        .output()
        .expect("Failed to execute command!");
    let output = String::from_utf8_lossy(&command.stdout).to_string();
    let status = command.status;
    let error = String::from_utf8_lossy(&command.stderr).to_string();
    let input_message = InputMessage::html(format!(
        "<code>{}</code>\n\n<b>{}</b>\n\n<code>{}</code>",
        output.trim(),
        status,
        error.trim()
    ));
    message.reply(input_message).await?;
    return Ok(());
}
