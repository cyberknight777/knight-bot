//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

mod aur;
mod cat;
mod dog;
mod eightball;
mod flipcoin;
mod help;
mod ipa;
mod link;
mod luck;
mod magisk;
mod man;
mod msg;
mod neo;
mod ping;
mod plant;
mod req;
mod run;
mod start;
mod uid;
mod urb;
mod webshot;
mod whois;

use grammers_client::{
    types::{Message},
    Client, Update
};
use getrandom::getrandom;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

enum Command {
    Aur(String),
    Cat(i64),
    Dog(i64),
    EightBall,
    FlipCoin,
    Help,
    Ipa(String),
    Link(String),
    Luck,
    Magisk(String),
    Man(String),
    Msg(String),
    Neo,
    Ping,
    Plant(i64),
    Run,
    Start,
    Uid,
    Urb(String),
    Webshot(String),
    Whois(String),
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
	"/aur" | "/aur@theknight_test_bot" => Command::Aur(args.join(" ")),
	"/cat" | "/cat@theknight_test_bot" => Command::Cat(args.join(" ").parse().unwrap_or_default()),
	"/dog" | "/dog@theknight_test_bot" => Command::Dog(args.join(" ").parse().unwrap_or_default()),
	"/eightball" | "/eightball@theknight_test_bot" => Command::EightBall,
	"/flipcoin" | "/flipcoin@theknight_test_bot" => Command::FlipCoin,
	"/help" | "/help@theknight_test_bot" => Command::Help,
	"/ipa" | "/ipa@theknight_test_bot" => Command::Ipa(args.join(" ")),
	"/link" | "/link@theknight_test_bot" => Command::Link(args.join(" ")),
	"/l" | "/l@theknight_test_bot" => Command::Luck,
	"/magisk" | "/magisk@theknight_test_bot" => Command::Magisk(args.join(" ")),
	"/man" | "/man@theknight_test_bot" => Command::Man(args.join(" ")),
	"/msg" | "/msg@theknight_test_bot" => Command::Msg(args.join(" ")),
	"/neo" | "/neo@theknight_test_bot" => Command::Neo,
	"/ping" | "/ping@theknight_test_bot" => Command::Ping,
	"/plant" | "/plant@theknight_test_bot" => Command::Plant(args.join(" ").parse().unwrap_or_default()),
	"/run" | "/run@theknight_test_bot" => Command::Run,
	"/start" | "/start@theknight_test_bot" => Command::Start,
	"/uid" | "/uid@theknight_test_bot" => Command::Uid,
	"/urb" | "/urb@theknight_test_bot" => Command::Urb(args.join(" ")),
	"/webshot" | "/webshot@theknight_test_bot" => Command::Webshot(args.join(" ")),
	"/whois" | "/whois@theknight_test_bot" => Command::Whois(args.join(" ")),
	_ => return Ok(()),
    };

    match cmd {
	Command::Aur(pkg) => aur::knightcmd_aur(message, pkg).await?,
	Command::Cat(kat) => cat::knightcmd_cat(client, message, kat).await?,
	Command::Dog(doge) => dog::knightcmd_dog(client, message, doge).await?,
	Command::EightBall => eightball::knightcmd_eightball(client, message).await?,
	Command::FlipCoin => flipcoin::knightcmd_flipcoin(client, message).await?,
	Command::Help => help::knightcmd_help(message).await?,
	Command::Ipa(addr) => ipa::knightcmd_ipa(message, addr).await?,
	Command::Link(url) => link::knightcmd_link(message, url).await?,
	Command::Luck => luck::knightcmd_luck(client, message).await?,
	Command::Magisk(var) => magisk::knightcmd_magisk(message, var).await?,
	Command::Man(cmd) => man::knightcmd_man(client, message, cmd).await?,
	Command::Msg(text) => msg::knightcmd_msg(client, message, text).await?,
	Command::Neo => neo::knightcmd_neo(client, message).await?,
	Command::Ping => ping::knightcmd_ping(message).await?,
	Command::Plant(plants) => plant::knightcmd_plant(client, message, plants).await?,
	Command::Run => run::knightcmd_run(message).await?,
	Command::Start => start::knightcmd_start(message).await?,
	Command::Uid => uid::knightcmd_uid(client, message).await?,
	Command::Urb(word) => urb::knightcmd_urb(message, word).await?,
	Command::Webshot(web) => webshot::knightcmd_webshot(client, message, web).await?,
	Command::Whois(site) => whois::knightcmd_whois(message, site).await?
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
