//!
//! Copyright (C) 2023-2025 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends cat pic according to the HTTP status code.

use grammers_client::{
    Client,
    message::{InputMessage, Message},
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_cat(client: Client, message: &Message, mut kat: i64) -> Result {
    if kat == 0 {
        kat = 404;
    }
    let url = format!("https://httpcats.com/{}.jpg", kat);
    let photo = InputMessage::new().text("").photo_url(url);
    client
        .send_message(message.peer_ref().await.unwrap(), photo)
        .await?;
    return Ok(());
}
