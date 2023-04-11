//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

use grammers_client::{
    Client,
    types::{InputMessage, Message}
};
use std::process::{Command, ExitStatus};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_webshot(client: Client, message: Message, web: String) -> Result {
    if web.trim().is_empty() {
	message.reply(InputMessage::html("Send me a <b>link of a website</b> to take a webshot of!")).await?;
	return Ok(());
    } else if !web.starts_with("http://") && !web.starts_with("https://") {
	message.reply(InputMessage::html("<b>Invalid URL!</b>")).await?;
	return Ok(());
    } else {
	let cmd: ExitStatus = Command::new("CutyCapt").arg(format!("--url={}", web)).arg("--out=ss.png").status().expect("Failed to run cutycapt");
	if cmd.success() {
	    let photo = client.upload_file("ss.png").await?;
	    client.send_message(message.chat(), InputMessage::text("Check this out!").photo(photo)).await?;
	} else {
	    message.reply(InputMessage::html("<b>Error! Couldn't take webshot.</b>")).await?;
	}
    }
    return Ok(());
}
