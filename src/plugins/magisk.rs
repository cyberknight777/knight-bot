//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Gets the latest Magisk release according to the variant

use crate::plugins;
use grammers_client::types::{InputMessage, Message};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_magisk(message: Message, var: String) -> Result {
    if var.trim().is_empty() {
	message.reply(InputMessage::html("Provide a <b>Magisk variant</b> to check for!
Supported variants are: stable, beta, canary")).await?;
	return Ok(());
    } else {
	let msg = message
	    .reply(InputMessage::html(format!("<b>Getting latest Magisk {} release...</b>", var)))
	    .await?;
	let url = format!("https://raw.githubusercontent.com/topjohnwu/magisk-files/master/{}.json", var);
	let response = plugins::req::make_request(url.to_string()).await;
	match response {
	    Some(response) => {
		let link = &response["magisk"]["link"];
		let version = &response["magisk"]["version"];
		msg.edit(InputMessage::markdown(format!("Latest {} release: [{}]({})", var, version.to_string().trim_matches('"').to_string(), link.to_string().trim_matches('"').to_string()))).await?;
	    },
	    None => {
		msg.edit("Failed to get Magisk release information!").await?;
	    },
	}	
    }
    return Ok(());
}
