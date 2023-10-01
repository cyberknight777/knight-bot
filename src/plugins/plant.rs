//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends plant pic according to http code

use grammers_client::{
    Client,
    types::{InputMessage, Message}
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_plant(client: Client, message: Message, mut plants: i64) -> Result {
    if plants == 0 { plants = 404; }
    let url = format!("https://http.garden/{}.jpg", plants);
    let photo = InputMessage::text("").photo_url(url);
    client.send_message(message.chat(), photo).await?;
    return Ok(());
}
