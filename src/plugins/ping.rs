//
// Copyright (C) 2023 cyberknight777
//
// SPDX-License-Identifier: MIT
//

// Import teloxide framework.
use std::thread::sleep;
use std::time;
use teloxide::prelude::*;

// A function that sends a message with the time taken to send the message.
pub async fn knightcmd_ping(bot: Bot, msg: Message) -> ResponseResult<()> {
    let start = time::Instant::now();
    sleep(time::Duration::new(1,0));
    let end = time::Instant::now();
    let differ = end.duration_since(start).as_millis();
    let duration = differ / 10;
    bot.send_message(msg.chat.id, format!("Pong! ~> {}ms", duration)).await?;
    Ok(())
}
	
