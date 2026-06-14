//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends a RTFM text.

use grammers_client::message::{Button, InputMessage, Message, ReplyMarkup};

type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn knightcmd_rtfm(message: &Message) -> Result {
    if let Some(id) = message.reply_to_message_id() {
        message
            .respond(
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
