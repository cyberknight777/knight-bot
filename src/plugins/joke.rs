//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends a random joke if the type is not provided. Available types: Misc, Programming, Dark, Pun, Spooky, Christmas.

use crate::plugins;
use grammers_client::message::{InputMessage, Message};

type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn knightcmd_joke(message: &Message, mut typej: String) -> Result {
    let api = "https://v2.jokeapi.dev/joke";

    typej = match typej.trim().to_ascii_lowercase().as_str() {
        "" => "Any".to_string(),
        "misc" => "Misc".to_string(),
        "programming" => "Programming".to_string(),
        "dark" => "Dark".to_string(),
        "pun" => "pun".to_string(),
        "spooky" => "Spooky".to_string(),
        "christmas" => "Christmas".to_string(),
        _ => {
            message
                .reply(
                    "Invalid joke type. Available: Misc, Programming, Dark, Pun, Spooky, Christmas",
                )
                .await?;
            return Ok(());
        }
    };

    let resp = match plugins::req::make_request(format!("{}/{}", api.to_string(), typej)).await {
        Some(r) => r,
        None => {
            message.reply("JokeAPI request failed").await?;
            return Ok(());
        }
    };

    match resp["type"].as_str().unwrap_or("Unknown type") {
        "single" => {
            message
                .reply(resp["joke"].as_str().unwrap_or("Unknown joke").to_string())
                .await?;
        }
        "twopart" => {
            message
                .reply(InputMessage::new().html(format!(
                        "{}\n...{}",
                        resp["setup"]
                            .as_str()
                            .unwrap_or("Unknown setup")
                            .to_string(),
                        resp["delivery"]
                            .as_str()
                            .unwrap_or("Unknown setup")
                            .to_string()
                    )))
                .await?;
        }
        _ => return Ok(()),
    }
    return Ok(());
}
