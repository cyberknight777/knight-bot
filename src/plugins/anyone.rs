//!
//! Copyright (C) 2023-2025 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends a why do you ask text.

use grammers_client::{
    button, reply_markup,
    types::{InputMessage, update::Message},
    Client,
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_anyone(client: Client, message: Message) -> Result {
    if let Some(id) = message.reply_to_message_id() {
        client
            .send_message(
                message.peer().unwrap(),
                InputMessage::html(message.html_text(), "Hmm.")
                    .reply_to(Some(id))
                    .reply_markup(&reply_markup::inline(vec![vec![button::url(
                        "Why do you ask?",
                        "https://dontasktoask.com",
                    )]])),
            )
            .await?;
    } else {
        message
            .reply(
                InputMessage::html("Hmm.").reply_markup(&reply_markup::inline(vec![vec![
                    button::url("Why do you ask?", "https://dontasktoask.com"),
                ]])),
            )
            .await?;
    }
    return Ok(());
}
