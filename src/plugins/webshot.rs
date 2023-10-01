//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Takes a webshot of a website

use crate::plugins;
use grammers_client::{
    Client,
    types::{InputMessage, Message}
};
use std::fs;
use std::process::{Command, ExitStatus};
use std::path::Path;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_webshot(client: Client, message: Message, web: String) -> Result {
    if web.trim().is_empty() {
	message.reply(InputMessage::html("Send me a <b>link of a website</b> to take a webshot of!")).await?;
	return Ok(());
    } else if !web.starts_with("http://") && !web.starts_with("https://") {
	message.reply(InputMessage::html("<b>Invalid URL!</b>")).await?;
	return Ok(());
    } else {
	let rng = plugins::random(6); // modulo 6 to get a number between 0 and 5
	let filename = format!("{}.png", rng);
	let cmd: ExitStatus = Command::new("CutyCapt").arg(format!("--url={}", web)).arg(format!("--out={}", Path::new(&filename).display())).status().expect("Failed to run cutycapt");
	if cmd.success() {
	    let photo = client.upload_file(Path::new(&filename)).await?;
	    message.reply(InputMessage::text("Check this out!").photo(photo)).await?;
	} else {
	    message.reply(InputMessage::html("<b>Error! Couldn't take webshot.</b>")).await?;
	}
	let _ = fs::remove_file(Path::new(&filename));
    }
    return Ok(());
}
