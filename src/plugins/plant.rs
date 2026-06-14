//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends plant pic according to http code.

use grammers_client::message::{InputMessage, Message};

type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn knightcmd_plant(message: &Message, mut plants: i64) -> Result {
    if plants == 0 {
        plants = 404;
    }
    let url = format!("https://http.garden/{}.jpg", plants);
    let photo = InputMessage::new().text("").photo_url(url);
    message.respond(photo).await?;
    return Ok(());
}
