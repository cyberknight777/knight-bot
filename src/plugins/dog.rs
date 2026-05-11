//!
//! Copyright (C) 2023-2025 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends dog pic according to the HTTP status code.

use grammers_client::{
    Client,
    message::{InputMessage, Message},
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_dog(client: Client, message: &Message, mut doge: i64) -> Result {
    if doge == 0 {
        doge = 404;
    }
    let url = format!("https://http.dog/{}.jpg", doge);
    let photo = InputMessage::new().text("").photo_url(url);
    client
        .send_message(message.peer_ref().await.unwrap(), photo)
        .await?;
    return Ok(());
}
