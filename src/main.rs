//
// Copyright (C) 2023 cyberknight777
//
// SPDX-License-Identifier: MIT
//

// Import teloxide framework.
// Also import the plugins module which is located in src/plugins.
mod plugins;
use teloxide::{prelude::*, utils::command::BotCommands};

// The main function where the bot is initialized.
#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting knight-bot...");

    let bot = Bot::from_env();

    Tgcmd::repl(bot, answer).await;

}

// An enumerator used to define commands that are accepted by the bot.
//
// @help - Replies with the accepted commands.
// @msg - Replies with the text that is sent as a parameter.
// @ping - Replies with a Pong and the time took to execute the command.
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Tgcmd {
    #[command(description = "display this text.")]
    Help,

    #[command(description = "Sends text.")]
    Msg(String),

    #[command(description = "Checks alive/dead.")]
    Ping,
}

// A function used to specify on what the accepted commands do.
async fn answer(bot: Bot, msg: Message, cmd: Tgcmd) -> ResponseResult<()> {
    match cmd {
	Tgcmd::Help => plugins::help::knightcmd_help(bot, msg).await?,
	Tgcmd::Msg(text) => plugins::msg::knightcmd_msg(bot, msg, text).await?,
	Tgcmd::Ping => plugins::ping::knightcmd_ping(bot, msg).await?
    };

    Ok(())
}
