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
	/aur [package] ~ Gets package information from AUR.\n\
	/cat [http code] ~ Sends cat pic according to http codes.\n\
	/dog [http code] ~ Sends dog pic according to http codes.\n\
	/eightball ~ Rolls an eightball to say yes/no.\n\
	/flipcoin ~ Flips a coin to say heads/tails.\n\
	/help ~ Displays this text.\n\
	/ipa [ip] ~ Gets information of given IP.\n\
	/link [url] ~ Gets the last redirected URL.\n\
	/l ~ To say your lucky number.\n\
	/magisk [stable/beta/canary] ~ Gets download link of latest Magisk according to variant.\n\
	/man [command] ~ Gets information of a command from manpages.\n\
	/msg [msg] ~ Sends text.\n\
	/neo ~ Sends neofetch output. \n\
	/ping ~ Checks how fast I can respond.\n\
	/plant [http code] ~ Sends plant pic according to http codes.\n\
	/run ~ Runnns :).\n\
	/start ~ Checks if I'm alive.\n\
	/uid ~ Gets UserID & ChatID>.\n\
	/urb [word] ~ Gets definition of word from urban dictionary.\n\
	/whois [site] ~ Gets WHOIS information of site.";
    message.reply(msg).await?;
    return Ok(());
}
