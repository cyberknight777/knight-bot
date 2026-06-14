//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Extracts redirected URL from given link.

use grammers_client::message::{InputMessage, Message};
use reqwest::header::LOCATION;

type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn knightcmd_link(message: &Message, url: String) -> Result {
    if url.trim().is_empty() {
        message
            .reply(InputMessage::new().html("Send a <b>proper URL</b>!"))
            .await?;
        return Ok(());
    } else if !url.starts_with("http://") && !url.starts_with("https://") {
        message
            .reply(InputMessage::new().html("<b>Invalid URL!</b>"))
            .await?;
        return Ok(());
    } else {
        let msg = message
            .reply(InputMessage::new().html("<b>Extracting redirected URL from given link...</b>"))
            .await?;
        let req = reqwest::Client::new();
        let mut response = req.head(url).send().await?;
        while response.status().is_redirection() {
            if let Some(location) = response.headers().get(LOCATION) {
                let location_str = location.to_str().unwrap_or_default();
                response = req.head(location_str).send().await?;
            } else {
                msg.edit(
                    InputMessage::new().html("<b>Error! Could not extract redirected URL!</b>"),
                )
                .await?;
                return Ok(());
            }
        }
        if response.status().is_success() {
            msg.edit(response.url().as_str()).await?
        } else {
            msg.edit(InputMessage::new().html("<b>Error! Could not extract redirected URL!</b>"))
                .await?;
        }
    }
    return Ok(());
}
