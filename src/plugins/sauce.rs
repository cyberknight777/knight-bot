//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Provides the link to the source code of this bot.

use grammers_client::{
    button, reply_markup,
    types::{InputMessage, Message},
    Client,
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_sauce(client: Client, message: Message) -> Result {
    if let Some(id) = message.reply_to_message_id() {
        client
            .send_message(
                message.chat(),
                InputMessage::html("You asked for it, so here you go!")
                    .reply_to(Some(id))
                    .reply_markup(&reply_markup::inline(vec![vec![button::url(
                        "sauce",
                        "https://github.com/cyberknight777/knight-bot",
                    )]])),
            )
            .await?;
    } else {
        message
            .reply(
                InputMessage::html("You asked for it, so here you go!").reply_markup(
                    &reply_markup::inline(vec![vec![button::url(
                        "sauce",
                        "https://github.com/cyberknight777/knight-bot",
                    )]]),
                ),
            )
            .await?;
    }
    return Ok(());
}
