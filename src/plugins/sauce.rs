//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Provides the link to the source code of this bot.

use grammers_client::{
    Client,
    message::{Button, InputMessage, Message, ReplyMarkup},
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_sauce(client: Client, message: &Message) -> Result {
    if let Some(id) = message.reply_to_message_id() {
        client
            .send_message(
                message.peer_ref().await.unwrap(),
                InputMessage::new()
                    .html("You asked for it, so here you go!")
                    .reply_to(Some(id))
                    .reply_markup(ReplyMarkup::from_buttons(&vec![vec![Button::url(
                        "sauce",
                        "https://github.com/cyberknight777/knight-bot",
                    )]])),
            )
            .await?;
    } else {
        message
            .reply(
                InputMessage::new()
                    .html("You asked for it, so here you go!")
                    .reply_markup(ReplyMarkup::from_buttons(&vec![vec![Button::url(
                        "sauce",
                        "https://github.com/cyberknight777/knight-bot",
                    )]])),
            )
            .await?;
    }
    return Ok(());
}
