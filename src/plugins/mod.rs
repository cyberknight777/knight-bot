//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

mod eightball;
mod flipcoin;
mod help;
mod link;
mod luck;
mod msg;
mod neo;
mod ping;
mod start;
mod urb;

use grammers_client::{
    types::{Message},
    Client, Update
};
use getrandom::getrandom;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

enum Command {
    EightBall,
    FlipCoin,
    Help,
    Link(String),
    Luck,
    Msg(String),
    Neo,
    Ping,
    Start,
    Urb(String),
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
	"/link" | "/link@theknight_test_bot" => Command::Link(args.join(" ")),
	"/l" | "/l@theknight_test_bot" => Command::Luck,
	"/msg" | "/msg@theknight_test_bot" => Command::Msg(args.join(" ")),
	"/neo" | "/neo@theknight_test_bot" => Command::Neo,
	"/ping" | "/ping@theknight_test_bot" => Command::Ping,
	"/start" | "/start@theknight_test_bot" => Command::Start,
	"/urb" | "/urb@theknight_test_bot" => Command::Urb(args.join(" ")),
	_ => return Ok(()),
    };

    match cmd {
	Command::EightBall => eightball::knightcmd_eightball(message).await?,
	Command::FlipCoin => flipcoin::knightcmd_flipcoin(message).await?,
	Command::Help => help::knightcmd_help(client, message).await?,
	Command::Link(url) => link::knightcmd_link(message, url).await?,
	Command::Luck => luck::knightcmd_luck(message).await?,
	Command::Msg(text) => msg::knightcmd_msg(client, message, text).await?,
	Command::Neo => neo::knightcmd_neo(client, message).await?,
	Command::Ping => ping::knightcmd_ping(message).await?,
	Command::Start => start::knightcmd_start(client, message).await?,
	Command::Urb(word) => urb::knightcmd_urb(message, word).await?
    }

    Ok(())
}

fn check_msg(message: &Message) -> bool {
    return !message.outgoing() && message.text().starts_with('/') && !message.text().starts_with("/ ") || message.text().ends_with("@theknight_test_bot");
}

pub fn random(modulo: u8) -> u8 {
    let mut buffer = [0; 1];
    getrandom(&mut buffer).expect("Failed to generate random number");
    return buffer[0] % modulo;
}
