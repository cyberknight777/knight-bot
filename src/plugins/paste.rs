//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends a pastebin link of the replied message or the text given

use grammers_client::{
    Client,
    types::{InputMessage, Message}
};
use librustbin::Client as RbinClient;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

fn check_paste(url: &str) -> bool {

    !url.is_empty() || url != "This file is empty!" || url != "This file exceeds the file limit"
}

pub async fn knightcmd_paste(client: Client, message: Message, past: String) -> Result {
    let msg = message
	.reply(InputMessage::html(format!("<b>Pasting content...</b>")))
	.await?;
    if let Some(reply) = client.get_reply_to_message(&message).await? {
	if !reply.text().is_empty() {
	    let url = format!(
		"{}",
		RbinClient::new("https://bin.cyberknight777.dev".to_string())
		    .paste_highlight(reply.text().to_string())
		    .unwrap()
		    .trim(),
	    );
	    if check_paste(&url) {
		msg.edit(InputMessage::html(format!("Link: {}", url))).await?;
	    } else {
		msg.edit(InputMessage::html("<b>Paste failed!</b>")).await?;
	    }
	} else {
	    msg.edit(InputMessage::html("<b>Paste failed!</b>")).await?;
	}
    } else if !past.is_empty() {
	let url = format!(
	    "{}",
	    RbinClient::new("https://bin.cyberknight777.dev".to_string())
		.paste_highlight(past)
		.unwrap()
		.trim(),
	);
	if check_paste(&url) {
	    msg.edit(InputMessage::html(format!("Link: {}", url))).await?;
	} else {
	    msg.edit(InputMessage::html("<b>Paste failed!</b>")).await?;
	}
    } else {
	msg.edit(InputMessage::html("Please reply to a <b>message</b> or reply with <b>/paste yourtext</b> to paste it!")).await?;
    }

    return Ok(());
}
