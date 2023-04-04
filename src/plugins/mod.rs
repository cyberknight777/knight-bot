//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

mod eightball;
mod flipcoin;
mod help;
mod luck;
mod msg;
mod ping;
mod start;

use grammers_client::{
    types::{Message},
    Client, Update
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

enum Command {
    EightBall,
    FlipCoin,
    Help,
    Luck,
    Msg(String),
    Ping,
    Start,
}

pub async fn handle_update(client: Client, update: Update) -> Result {
    match update {
        Update::NewMessage(message) if check_msg(&message) => {
            log::info!("Responding to {}", message.chat().name());
            handle_msg(client, message).await?
        }
        _ => {}
    }

    Ok(())
}

pub async fn handle_msg(client: Client, message: Message) -> Result {
    let msg = message.text();
    let _chat = message.chat(); // It is unused for the moment.
    let cmd = msg.split_whitespace().next().unwrap();
    let args = msg.split_whitespace().skip(1).collect::<Vec<_>>();
    let cmd = match cmd {
	"/eightball" | "/eightball@theknight_test_bot" => Command::EightBall,
	"/flipcoin" | "/flipcoin@theknight_test_bot" => Command::FlipCoin,
	"/help" | "/help@theknight_test_bot" => Command::Help,
	"/l" | "/l@theknight_test_bot" => Command::Luck,
	"/msg" | "/msg@theknight_test_bot" => Command::Msg(args.join(" ")),
	"/ping" | "/ping@theknight_test_bot" => Command::Ping,
	"/start" | "/start@theknight_test_bot" => Command::Start,
	_ => return Ok(()),
    };

    match cmd {
	Command::EightBall => eightball::knightcmd_eightball(client, message).await?,
	Command::FlipCoin => flipcoin::knightcmd_flipcoin(client, message).await?,
	Command::Help => help::knightcmd_help(client, message).await?,
	Command::Luck => luck::knightcmd_luck(client, message).await?,
	Command::Msg(text) => msg::knightcmd_msg(client, message, text).await?,
	Command::Ping => ping::knightcmd_ping(client, message).await?,
	Command::Start => start::knightcmd_start(client, message).await?
    }

    Ok(())
}

fn check_msg(message: &Message) -> bool {
    return !message.outgoing() && message.text().starts_with('/') && !message.text().starts_with("/ ") || message.text().ends_with("@theknight_test_bot");
}
