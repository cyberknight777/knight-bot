//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends cat pic according to the HTTP status code.

use grammers_client::message::{InputMessage, Message};

type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn knightcmd_cat(message: &Message, mut kat: i64) -> Result {
    if kat == 0 {
        kat = 404;
    }
    let url = format!("https://httpcats.com/{}.jpg", kat);
    let photo = InputMessage::new().text("").photo_url(url);
    message.reply(photo).await?;
    return Ok(());
}
