//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends dog pic according to the HTTP status code.

use grammers_client::message::{InputMessage, Message};

type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn knightcmd_dog(message: &Message, mut doge: i64) -> Result {
    if doge == 0 {
        doge = 404;
    }
    let url = format!("https://http.dog/{}.jpg", doge);
    let photo = InputMessage::new().text("").photo_url(url);
    message.respond(photo).await?;
    return Ok(());
}
