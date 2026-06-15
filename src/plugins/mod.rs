//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

use crate::cfg;
use std::sync::Arc;

mod anyone;
mod aur;
mod cat;
mod dl;
mod dog;
mod eightball;
mod fact;
mod flipcoin;
mod help;
mod ipa;
mod joke;
mod link;
mod lpaste;
mod luck;
mod magisk;
mod man;
mod mot;
mod msg;
mod neo;
mod paste;
mod ping;
mod plant;
mod req;
mod rtfm;
mod run;
mod sauce;
mod sh;
mod smsg;
mod start;
mod uid;
mod ul;
mod urb;
mod whois;
mod yaap;

use getrandom;
use grammers_client::{Client, message::Message, update::Update};

type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

enum Command {
    Anyone,
    Aur(String),
    Cat(i64),
    Dl(String),
    Dog(i64),
    EightBall,
    Fact,
    FlipCoin,
    Help(String),
    Ipa(String),
    Joke(String),
    Link(String),
    Lpaste(String),
    Luck,
    Magisk,
    Man(String),
    Mot(String, String, String),
    Msg(String),
    Neo,
    Paste(String),
    Ping,
    Plant(i64),
    Rtfm,
    Run,
    Sauce,
    Sh(String),
    Smsg(String),
    Start,
    Uid,
    Ul(String),
    Urb(String),
    Whois(String),
    Yaap(String),
}

pub async fn handle_update(client: Client, update: Update, bot_username: &str) -> Result {
    let config = Arc::new(cfg::Config::read().expect("cannot read the config"));
    match update {
        Update::NewMessage(message)
            if check_msg(&message, bot_username)
                || check_cmd(&message, config.clone().admin_id) =>
        {
            log::info!(
                "Responding to {}",
                message.peer().and_then(|p| p.name()).unwrap_or("")
            );
            handle_msg(client, &message, bot_username).await?
        }
        _ => {}
    }

    Ok(())
}

pub async fn handle_msg(client: Client, message: &Message, bot_username: &str) -> Result {
    let msg = message.text();
    let cmd = msg.split_whitespace().next().unwrap_or("");
    let args = msg.split_whitespace().skip(1).collect::<Vec<_>>();
    let cmd = match bot_command(cmd, bot_username).unwrap_or(cmd) {
        "/anyone" => Command::Anyone,
        "/aur" => Command::Aur(args.join(" ")),
        "/cat" => Command::Cat(args.join(" ").parse().unwrap_or_default()),
        "k.dl" => Command::Dl(args.join(" ")),
        "/dog" => Command::Dog(args.join(" ").parse().unwrap_or_default()),
        "/eightball" => Command::EightBall,
        "/fact" => Command::Fact,
        "/flipcoin" => Command::FlipCoin,
        "/help" => Command::Help(args.join(" ")),
        "/ipa" => Command::Ipa(args.join(" ")),
        "/joke" => Command::Joke(args.join(" ")),
        "/link" => Command::Link(args.join(" ")),
        "/lpaste" => Command::Lpaste(args.join(" ")),
        "/luck" => Command::Luck,
        "/magisk" => Command::Magisk,
        "/man" => Command::Man(args.join(" ")),
        "/msg" => Command::Msg(args.join(" ")),
        "/neo" => Command::Neo,
        "/ping" => Command::Ping,
        "/paste" => Command::Paste(args.join(" ")),
        "/plant" => Command::Plant(args.join(" ").parse().unwrap_or_default()),
        "/rtfm" => Command::Rtfm,
        "/run" => Command::Run,
        "/sauce" => Command::Sauce,
        "/smsg" => Command::Smsg(args.join(" ")),
        "/start" => Command::Start,
        "/uid" => Command::Uid,
        "k.ul" => Command::Ul(args.join(" ")),
        "/urb" => Command::Urb(args.join(" ")),
        "/whois" => Command::Whois(args.join(" ")),
        "/yaap" => Command::Yaap(args.join(" ")),
        "k.sh" => Command::Sh(args.join(" ").parse().unwrap_or_default()),
        "k.mot" => Command::Mot(
            args.get(0).unwrap_or(&"").to_string(),
            args.get(1).unwrap_or(&"").to_string(),
            args.get(2).unwrap_or(&"").to_string(),
        ),
        _ => return Ok(()),
    };

    match cmd {
        Command::Anyone => anyone::knightcmd_anyone(message).await?,
        Command::Aur(pkg) => aur::knightcmd_aur(message, pkg).await?,
        Command::Cat(kat) => cat::knightcmd_cat(message, kat).await?,
        Command::Dl(link) => dl::knightcmd_dl(client, message, link).await?,
        Command::Dog(doge) => dog::knightcmd_dog(message, doge).await?,
        Command::EightBall => eightball::knightcmd_eightball(message).await?,
        Command::Fact => fact::knightcmd_fact(message).await?,
        Command::FlipCoin => flipcoin::knightcmd_flipcoin(message).await?,
        Command::Help(hcmd) => help::knightcmd_help(message, hcmd).await?,
        Command::Ipa(addr) => ipa::knightcmd_ipa(message, addr).await?,
        Command::Joke(typej) => joke::knightcmd_joke(message, typej).await?,
        Command::Link(url) => link::knightcmd_link(message, url).await?,
        Command::Lpaste(link) => lpaste::knightcmd_lpaste(message, link).await?,
        Command::Luck => luck::knightcmd_luck(message).await?,
        Command::Magisk => magisk::knightcmd_magisk(message).await?,
        Command::Man(cmd) => man::knightcmd_man(message, cmd).await?,
        Command::Mot(kuid, kcar, ksn) => {
            mot::knightcmd_mot(message, Some(kuid), Some(kcar), Some(ksn)).await?
        }
        Command::Msg(text) => msg::knightcmd_msg(message, text).await?,
        Command::Neo => neo::knightcmd_neo(message).await?,
        Command::Ping => ping::knightcmd_ping(message).await?,
        Command::Paste(past) => paste::knightcmd_paste(client, message, past).await?,
        Command::Plant(plants) => plant::knightcmd_plant(message, plants).await?,
        Command::Rtfm => rtfm::knightcmd_rtfm(message).await?,
        Command::Run => run::knightcmd_run(message).await?,
        Command::Sauce => sauce::knightcmd_sauce(message).await?,
        Command::Sh(kcmd) => sh::knightcmd_sh(message, kcmd).await?,
        Command::Smsg(stext) => smsg::knightcmd_smsg(message, stext).await?,
        Command::Start => start::knightcmd_start(message).await?,
        Command::Uid => uid::knightcmd_uid(message).await?,
        Command::Ul(path) => ul::knightcmd_ul(client, message, path).await?,
        Command::Urb(word) => urb::knightcmd_urb(message, word).await?,
        Command::Whois(site) => whois::knightcmd_whois(message, site).await?,
        Command::Yaap(device) => yaap::knightcmd_yaap(message, device).await?,
    }

    Ok(())
}

fn bot_command<'a>(cmd: &'a str, bot_username: &str) -> Option<&'a str> {
    let Some((command, username)) = cmd.split_once('@') else {
        return Some(cmd);
    };

    username
        .eq_ignore_ascii_case(bot_username)
        .then_some(command)
}

fn check_msg(message: &Message, bot_username: &str) -> bool {
    let text = message.text();
    let cmd = text.split_whitespace().next().unwrap_or("");

    return !message.outgoing()
        && text.starts_with('/')
        && !text.starts_with("/ ")
        && bot_command(cmd, bot_username).is_some();
}

fn check_cmd(message: &Message, admin_id: i64) -> bool {
    return !message.outgoing()
        && (message.sender().and_then(|s| s.id().bare_id()) == Some(admin_id))
        && (message.text().starts_with("k.sh")
            || message.text().starts_with("k.mot")
            || message.text().starts_with("k.ul")
            || message.text().starts_with("k.dl"));
}

pub fn random(modulo: u8) -> u8 {
    let mut buffer = [0; 1];
    getrandom::fill(&mut buffer).expect("Failed to generate random number");
    return buffer[0] % modulo;
}
