//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Gets UserID and ChatID

use grammers_client::{
    Client,
    types::{InputMessage, Message}
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_uid(client: Client, message: Message) -> Result {
    if let Some(id) = message.reply_to_message_id() {
	if let Some(reply_to_msg) = client.get_reply_to_message(&message).await? {
	    if let Some(sender) = reply_to_msg.sender() {
		client.send_message(message.chat(), InputMessage::html(format!("Your ID: <code>{}</code>
ChatID: <code>-100{}</code>
{}'s ID: <code>{}</code>", message.sender().unwrap().id(), message.chat().id(), sender.name(), sender.id())).reply_to(Some(id))).await?;
	    }
	}
    } else {
	message.reply(InputMessage::html(format!("Your ID: <code>{}</code>
ChatID: <code>-100{}</code>", message.sender().unwrap().id(), message.chat().id()))).await?;
    }
    return Ok(());
}
