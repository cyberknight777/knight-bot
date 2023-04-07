//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

use grammers_client::types::Message;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_help(message: Message) -> Result {
    let msg = "Hello There!, \
	I am a bot made by cyberknight777 in Rust based on gramme.rs.\n\
	Here's a list of my commands:-\n\
	/eightball ~ Rolls an eightball to say yes/no.\n\
	/flipcoin ~ Flips a coin to say heads/tails.\n\
	/help ~ Displays this text.\n\
	/link [url] ~ Gets the last redirected URL.\n\
	/l ~ To say your lucky number.\n\
	/msg [msg] ~ Sends text.\n\
	/neo ~ Sends neofetch output. \n\
	/ping ~ Checks how fast I can respond.\n\
	/start ~ Checks if I'm alive.\n\
	/urb [word] ~ Gets definition of word from urban dictionary.\n";
    message.reply(msg).await?;
    return Ok(());
}
