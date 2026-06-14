//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends a RTFM text.

use grammers_client::{
    Client,
    message::{Button, InputMessage, Message, ReplyMarkup},
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_rtfm(client: Client, message: &Message) -> Result {
    if let Some(id) = message.reply_to_message_id() {
        client
            .send_message(
                message.peer_ref().await.unwrap(),
                InputMessage::new()
                    .html("How bout you...")
                    .reply_to(Some(id))
                    .reply_markup(ReplyMarkup::from_buttons(&vec![vec![Button::url(
                        "Read the fucking manual",
                        "https://readthefuckingmanual.com",
                    )]])),
            )
            .await?;
    } else {
        message
            .reply(InputMessage::new().html("How bout you...").reply_markup(
                ReplyMarkup::from_buttons(&vec![vec![Button::url(
                    "Read the fucking manual",
                    "https://readthefuckingmanual.com",
                )]]),
            ))
            .await?;
    }
    return Ok(());
}
