//!
//! Copyright (C) 2023-2025 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends dog pic according to the HTTP status code.

use grammers_client::{
    types::{InputMessage, Message},
    Client,
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_dog(client: Client, message: Message, mut doge: i64) -> Result {
    if doge == 0 {
        doge = 404;
    }
    let url = format!("https://http.dog/{}.jpg", doge);
    let photo = InputMessage::text("").photo_url(url);
    client.send_message(message.chat(), photo).await?;
    return Ok(());
}
