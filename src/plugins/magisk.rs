//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Gets the latest Magisk release according to the variant.

use crate::plugins;
use grammers_client::{
    button,
    Client,
    reply_markup,
    types::{InputMessage, Message}
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_magisk(client: Client, message: Message) -> Result {

    let stable = format!("https://raw.githubusercontent.com/topjohnwu/magisk-files/master/stable.json");
    let stable_resp = plugins::req::make_request(stable.to_string()).await;

    let beta = format!("https://raw.githubusercontent.com/topjohnwu/magisk-files/master/beta.json");
    let beta_resp = plugins::req::make_request(beta.to_string()).await;

    let canary = format!("https://raw.githubusercontent.com/topjohnwu/magisk-files/master/canary.json");
    let canary_resp = plugins::req::make_request(canary.to_string()).await;

    let stable_version;
    let stable_link;

    let beta_version;
    let beta_link;

    let canary_version;
    let canary_link;

    match stable_resp {
        Some(stable_resp) => {
	stable_link = (&stable_resp["magisk"]["link"]).to_string();
	stable_version = (&stable_resp["magisk"]["version"]).to_string();
        },
        None => {
	message.reply(InputMessage::html("Failed to get Magisk release information! (Stable)")).await?;
	return Ok(());
        },
    }
    match beta_resp {
        Some(beta_resp) => {
            beta_link = (&beta_resp["magisk"]["link"]).to_string();
            beta_version = (&beta_resp["magisk"]["version"]).to_string();
        },
        None => {
            message.reply(InputMessage::html("Failed to get Magisk release information! (Beta)")).await?;
	return Ok(());
        },
    }
    match canary_resp {
        Some(canary_resp) => {
            canary_link = (&canary_resp["magisk"]["link"]).to_string();
            canary_version = (&canary_resp["magisk"]["version"]).to_string();
        },
        None => {
            message.reply(InputMessage::html("Failed to get Magisk release information! (Canary)")).await?;
	return Ok(());
        },
    }
    if let Some(id) = message.reply_to_message_id() {
        client.send_message(message.chat(), InputMessage::html(format!("<b>Latest Magisk Releases</b>:")).reply_to(Some(id)).reply_markup(&reply_markup::inline(vec![
	vec![button::url(
	    format!("Stable: {}", stable_version.to_string().trim_matches('"').to_string()),
	    stable_link.to_string().trim_matches('"').to_string(),
	)],
            vec![button::url(
                format!("Beta: {}", beta_version.to_string().trim_matches('"').to_string()),
                beta_link.to_string().trim_matches('"').to_string(),
            )],
            vec![button::url(
                format!("Canary: {}", canary_version.to_string().trim_matches('"').to_string()),
                canary_link.to_string().trim_matches('"').to_string(),
            )],
        ]))).await?;
    } else {
        message.reply(InputMessage::html(format!("<b>Latest Magisk Releases</b>:")).reply_markup(&reply_markup::inline(vec![
	vec![button::url(
	    format!("Stable: {}", stable_version.to_string().trim_matches('"').to_string()),
	    stable_link.to_string().trim_matches('"').to_string(),
	)],
            vec![button::url(
                format!("Beta: {}", beta_version.to_string().trim_matches('"').to_string()),
                beta_link.to_string().trim_matches('"').to_string(),
            )],
            vec![button::url(
                format!("Canary: {}", canary_version.to_string().trim_matches('"').to_string()),
                canary_link.to_string().trim_matches('"').to_string(),
            )],
        ]))).await?;
    }
    return Ok(());
}
