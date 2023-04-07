//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

use grammers_client::types::Message;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_help(message: Message) -> Result {
    let msg = "Hello There!, \
	I am a bot made by cyberknight777 in Rust based on teloxide.\n\
	Here's a list of my commands:-\n\
	/eightball ~ Rolls an eightball to say yes/no.\n\
        /flipcoin ~ Flips a coin to say heads/tails.\n\
	/help ~ Displays this text.\n\
	/l ~ To say your lucky number.\n\
	/msg ~ Sends text.\n\
	/neo ~ Sends neofetch output. \n\
	/ping ~ Checks how fast I can respond.\n\
	/start ~ Checks if I'm alive.\n";
    message.reply(msg).await?;
    return Ok(());
}
