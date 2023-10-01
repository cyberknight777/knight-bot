//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Checks WHOIS information of a given URL

use grammers_client::types::{InputMessage, Message};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_whois(message: Message, site: String) -> Result {
    if site.trim().is_empty() {
	message.reply(InputMessage::html("Send a <b>proper URL</b> to get WHOIS information!")).await?;
	return Ok(());
    } else {
	let msg = message
	    .reply(InputMessage::html("<b>Extracting WHOIS information from given link...</b>"))
	    .await?;
	let mut whois_process = tokio::process::Command::new("whois")
            .arg(site)
            .stdout(std::process::Stdio::piped())
            .spawn()?;
	let mut grep_process = tokio::process::Command::new("rg")
            .arg("Registrar")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;
	if let ( Some(mut whois_stdout), Some(mut grep_stdin)) = (whois_process.stdout.take(), grep_process.stdin.take()) {
            let mut buffer = Vec::new();
            whois_stdout.read_to_end(&mut buffer).await?;
            grep_stdin.write_all(&buffer).await?;
            whois_process.stdout = Some(whois_stdout);
            grep_process.stdin = Some(grep_stdin);
	}
	let output = grep_process.wait_with_output().await?.stdout;
	if output.is_empty() {
	    msg.edit("No WHOIS information found!").await?;
	} else {
	    msg.edit(format!("{}", String::from_utf8_lossy(&output))).await?;
	}
    }
    return Ok(());
}
