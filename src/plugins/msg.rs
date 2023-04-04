//
// Copyright (C) 2023 cyberknight777
//
// SPDX-License-Identifier: MIT
//

// Import teloxide framework.
use teloxide::prelude::*;

// A function that sends a message with the parameter it is passed i.e /msg foo.
pub async fn knightcmd_msg(bot: Bot, msg: Message, text: String) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, format!("{text}")).await?;
    Ok(())
}
