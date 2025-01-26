//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends a RTFM text.

use grammers_client::{
    button,
    Client,
    reply_markup,
    types::{InputMessage, Message}
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_rtfm(client: Client, message: Message) -> Result {
    if let Some(id) = message.reply_to_message_id() {
        client.send_message(message.chat(), InputMessage::html("How bout you...").reply_to(Some(id)).reply_markup(&reply_markup::inline(vec![vec![
                button::url(
                        "Read the fucking manual",
                        "https://readthefuckingmanual.com",
                )
        ]]))).await?;
    } else {
        message.reply(InputMessage::html("How bout you...").reply_markup(&reply_markup::inline(vec![vec![
                button::url(
			"Read the fucking manual",
			"https://readthefuckingmanual.com",
		)
	]]))).await?;
    }
    return Ok(());
}
