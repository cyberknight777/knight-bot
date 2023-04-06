//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

use grammers_client::{
    Client,
    types::{InputMessage, Message}
};
use getrandom::getrandom;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_luck(_client: Client, message: Message) -> Result {
    let mut buffer = [0; 1];
    getrandom(&mut buffer).expect("Failed to generate random number");
    let random_number = buffer[0] % 101; // modulo 101 to get a number between 0 to 100
    message.reply(InputMessage::html(format!("Your lucky number is: <code>{}</code>", random_number))).await?;

    return Ok(());
}
