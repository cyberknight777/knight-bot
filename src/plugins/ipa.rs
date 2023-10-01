//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Sends info about an IP Address

use crate::plugins;
use grammers_client::types::{InputMessage, Message};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_ipa(message: Message, addr: String) -> Result {
    if addr.trim().is_empty() {
	message.reply(InputMessage::html("Send a <b>proper IP Address</b>!")).await?;
	return Ok(());
    } else {
	let msg = message
	    .reply(InputMessage::html("<b>Extracting info from ip addr........</b>"))
	    .await?;
	let url = format!("https://ipinfo.io/{}", addr);
        let response = plugins::req::make_request(url.to_string()).await;
        if response.is_none() {
            msg.edit("Something went wrong! Please try again").await?;
            return Ok(());
        } else if &response.clone().unwrap()["status"]
            .to_string()
            .trim_matches('"')
            .to_string()
            == "404"
        {
            msg.edit(InputMessage::html("Send a <b>proper IP Address</b>!")).await?;
	    return Ok(());
        } else {
            let hname = &response.clone().unwrap()["hostname"]
                .to_string()
                .trim_matches('"')
                .to_string();
            let city = &response.clone().unwrap()["city"]
                .to_string()
                .trim_matches('"')
                .to_string();
            let rgn = &response.clone().unwrap()["region"]
                .to_string()
                .trim_matches('"')
                .to_string();
            let ctry = &response.clone().unwrap()["country"]
                .to_string()
                .trim_matches('"')
                .to_string();
            let loc = &response.clone().unwrap()["loc"]
                .to_string()
                .trim_matches('"')
                .to_string();
            let org = &response.clone().unwrap()["org"]
                .to_string()
                .trim_matches('"')
                .to_string();
            let postal = &response.clone().unwrap()["postal"]
                .to_string()
                .trim_matches('"')
                .to_string();
            let tz = &response.clone().unwrap()["timezone"]
                .to_string()
                .trim_matches('"')
                .to_string();
            msg.edit(InputMessage::html(format!(
                "<b>IP</b>: <code>{}</code>
<b>Hostname</b>: {}
<b>City</b>: {}
<b>Region</b>: {}
<b>Country</b>: {}
<b>Lat/Long</b>: {}
<b>Org</b>: {}
<b>Postal</b>: {}
<b>Timezone</b>: {}",
                addr, hname, city, rgn, ctry, loc, org, postal, tz
            ))).await?;
	}
    }
    return Ok(());
}
