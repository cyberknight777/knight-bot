//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Runnns :)

use grammers_client::types::{InputMessage, Message};
use std::time::Instant;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_run(message: Message) -> Result {
    let start = Instant::now();
    let elapsed = start.elapsed();
    let sec = elapsed.subsec_nanos() % 3;
    let a = String::from("The winter dog is running......");
    let b = String::from("Run away and never come back......");
    let c = String::from("Let's keep running folks!");
    let msg;
    if sec == 0 {
	msg = a;
    } else if sec == 1 {
	msg = b;
    } else {
	msg = c;
    }
    message.reply(InputMessage::html(format!("<b>{}</b>", msg))).await?;
    return Ok(());
}
