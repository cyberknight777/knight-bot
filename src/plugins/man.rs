//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Gets information about a command from manpages

use grammers_client::{
    Client,
    types::{InputMessage, Message}
};
use std::process::Command;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_man(client: Client, message: Message, cmd: String) -> Result {
    if cmd.trim().is_empty() {
	message.reply(InputMessage::html("<code>Provide a command to check its manual entry!</code>")).await?;
	return Ok(());
    }
    let output = Command::new("man").arg("-f").arg(&cmd).output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);
    let msg = output_str.trim();
    if msg.is_empty() {
	message.reply("No manual entry found for this command.").await?;
	return Ok(());
    }
    let text = format!("<code>{}</code>", msg);
    if let Some(id) = message.reply_to_message_id() {
	client.send_message(message.chat(), InputMessage::html(text).reply_to(Some(id))).await?;
    } else {
	message.reply(InputMessage::html(text)).await?;
    }
    return Ok(());
}
