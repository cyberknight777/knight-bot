//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends a why do you ask text.

use grammers_client::message::{Button, InputMessage, Message, ReplyMarkup};

type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn knightcmd_anyone(message: &Message) -> Result {
    if let Some(id) = message.reply_to_message_id() {
        message
            .respond(
                InputMessage::new()
                    .html("Hmm.")
                    .reply_to(Some(id))
                    .reply_markup(ReplyMarkup::from_buttons(&vec![vec![Button::url(
                        "Why do you ask?",
                        "https://dontasktoask.com",
                    )]])),
            )
            .await?;
    } else {
        message
            .reply(
                InputMessage::new()
                    .html("Hmm.")
                    .reply_markup(ReplyMarkup::from_buttons(&vec![vec![Button::url(
                        "Why do you ask?",
                        "https://dontasktoask.com",
                    )]])),
            )
            .await?;
    }
    return Ok(());
}
