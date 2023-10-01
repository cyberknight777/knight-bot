//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Gets the definition of word from urban dictionary

use crate::plugins;
use grammers_client::types::{InputMessage, Message};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

async fn get_def(taxt: &String) -> Option<String> {
    let url = format!("https://api.urbandictionary.com/v0/define?term={}", taxt);
    let response = match plugins::req::make_request(url).await {
        Some(val) => val,
        None => return None,
    };
    if response["list"].as_array().unwrap().is_empty() {
        return Some(String::from("No definition found!"));
    }
    let target = &response["list"][0]["definition"];
    Some(target.to_string().trim_matches('"').to_string())
}

pub async fn knightcmd_urb(message: Message, word: String) -> Result {
    if word.trim().is_empty() {
	let msg = message
	    .reply(InputMessage::html("<b>Getting definition of random word from urban dictionary...</b>"))
	    .await?;
	let url = "http://api.urbandictionary.com/v0/random";
	let response = plugins::req::make_request(url.to_string()).await;
	let word = &response.clone().unwrap()["list"][0]["word"];
	let defin = &response.clone().unwrap()["list"][0]["definition"];
	msg.edit(InputMessage::html(format!("Definition for <b>{}</b> : <i>{}</i>", word.to_string().trim_matches('"').to_string(), defin.to_string().trim_matches('"').to_string().replace(r#"\r\n"#, "")))).await?;
    } else {
	let msg = message
	    .reply(InputMessage::html("<b>Getting definition of word from urban dictionary...</b>"))
	    .await?;
	let defin = get_def(&word).await;
	if defin.is_none() {
	    msg.edit("Something went wrong!").await?;
	} else {
	    msg.edit(InputMessage::html(format!("Definition for <b>{}</b> : <i>{}</i>", word, defin.unwrap().replace(r#"\r\n"#, "")))).await?;
	}
    }
    return Ok(());
}
