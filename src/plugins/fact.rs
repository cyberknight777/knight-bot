//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends a random fact.

use crate::plugins;
use grammers_client::message::Message;

type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn knightcmd_fact(message: &Message) -> Result {
    let rng = plugins::random(3);
    let (api, parser) = match rng {
        0 => ("https://catfact.ninja/fact", "fact"),
        1 => (
            "https://uselessfacts.jsph.pl/api/v2/facts/random?language=en",
            "text",
        ),
        2 => ("https://dogapi.dog/api/v2/facts", "dog"),
        _ => return Ok(()),
    };

    let resp = match plugins::req::make_request(api.to_string()).await {
        Some(r) => r,
        None => {
            message.reply("Request failed").await?;
            return Ok(());
        }
    };

    let value = match parser {
        "fact" => resp["fact"].as_str().unwrap_or("Unknown fact").to_string(),
        "text" => resp["text"].as_str().unwrap_or("Unknown fact").to_string(),
        "dog" => resp["data"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|obj| obj.get("attributes"))
            .and_then(|attr| attr.get("body"))
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown fact")
            .to_string(),
        _ => return Ok(()),
    };

    message.reply(value).await?;
    return Ok(());
}
