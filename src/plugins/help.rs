//
// Copyright (C) 2023 cyberknight777
//
// SPDX-License-Identifier: MIT
//

// Import teloxide framework.
use teloxide::prelude::*;

// A function that sends a message with a list of commands that are accepted by the bot.
pub async fn knightcmd_help(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, format!("Hello There!, 
I am a bot made by cyberknight777 in Rust based on teloxide.
Here's a list of my commands:-
/help ~ Display this text.
/msg ~ Sends text.
")).await?;
    Ok(())
}
