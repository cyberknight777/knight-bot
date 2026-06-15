//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Provides the link to the source code of this bot.

use grammers_client::message::{Button, InputMessage, Message, ReplyMarkup};

type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

const EXCLUDED: &[&str] = &["cfg", "init", "main", "mod", "req"];

pub async fn knightcmd_sauce(message: &Message, scmd: String) -> Result {
    let mut url = "https://github.com/cyberknight777/knight-bot".to_string();
    if !scmd.is_empty() && !EXCLUDED.contains(&scmd.as_str()) {
        url = format!("{url}/blob/master/src/plugins/{scmd}.rs");
    }

    if let Some(id) = message.reply_to_message_id() {
        message
            .respond(
                InputMessage::new()
                    .html("You asked for it, so here you go!")
                    .reply_to(Some(id))
                    .reply_markup(ReplyMarkup::from_buttons(&vec![vec![Button::url(
                        "sauce",
                        format!("{url}"),
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
                        format!("{url}"),
                    )]])),
            )
            .await?;
    }
    return Ok(());
}
