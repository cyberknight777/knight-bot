//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Rolls an eightball to say yes or no.

use crate::plugins;
use grammers_client::{
    Client,
    types::{InputMessage, Message}
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_eightball(client: Client, message: Message) -> Result {
    let ball = plugins::random(2);
    let result = if ball == 0 { "Yes, it is the truth!" } else { "No, this is a prepostrous lie!" };
    if let Some(id) = message.reply_to_message_id() {
	client.send_message(message.chat(), InputMessage::text(result).reply_to(Some(id))).await?;
    } else {
	message.reply(result).await?;
    }
    return Ok(());
}
