//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Flips a coin to say heads or tails.

use crate::plugins;
use grammers_client::{
    Client,
    types::{InputMessage, Message}
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_flipcoin(client: Client, message: Message) -> Result {
    let coin = plugins::random(2);
    let result = if coin == 0 { "Heads!" } else { "Tails!" };
    if let Some(id) = message.reply_to_message_id() {
	client.send_message(message.chat(), InputMessage::text(result).reply_to(Some(id))).await?;
    } else {
	message.reply(result).await?;
    }
    return Ok(());
}
