//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Gets package information from AUR.

use crate::plugins;
use grammers_client::types::{InputMessage, Message};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_aur(message: Message, pkg: String) -> Result {
    if !pkg.is_empty() {
	let response = plugins::req::make_request(format!("https://aur.archlinux.org/rpc/?v=5&type=info&arg={}", pkg)).await;
	if let Some(result) = response.and_then(|val| val.get("results").cloned()) {
	    if let Some(pkg_info) = result.get(0) {
		  let name = pkg_info.get("Name").and_then(|val| val.as_str()).unwrap_or_default();
                let ver = pkg_info.get("Version").and_then(|val| val.as_str()).unwrap_or_default();
                let desc = pkg_info.get("Description").and_then(|val| val.as_str()).unwrap_or_default();
                let url = pkg_info.get("URL").and_then(|val| val.as_str()).unwrap_or_default();
                let grp = pkg_info.get("Groups").and_then(|val| val.as_array()).map(|arr| arr.iter().map(|val| val.as_str().unwrap_or_default()).collect::<Vec<_>>()).unwrap_or_default();
                let lic = pkg_info.get("License").and_then(|val| val.as_array()).map(|arr| arr.iter().map(|val| val.as_str().unwrap_or_default()).collect::<Vec<_>>()).unwrap_or_default();
                let prov = pkg_info.get("Provides").and_then(|val| val.as_array()).map(|arr| arr.iter().map(|val| val.as_str().unwrap_or_default()).collect::<Vec<_>>()).unwrap_or_default();
                let dep = pkg_info.get("Depends").and_then(|val| val.as_array()).map(|arr| arr.iter().map(|val| val.as_str().unwrap_or_default()).collect::<Vec<_>>()).unwrap_or_default();
                let mkdep = pkg_info.get("MakeDepends").and_then(|val| val.as_array()).map(|arr| arr.iter().map(|val| val.as_str().unwrap_or_default()).collect::<Vec<_>>()).unwrap_or_default();
                let chkdep = pkg_info.get("CheckDepends").and_then(|val| val.as_array()).map(|arr| arr.iter().map(|val| val.as_str().unwrap_or_default()).collect::<Vec<_>>()).unwrap_or_default();
                let optdep = pkg_info.get("OptDepends").and_then(|val| val.as_array()).map(|arr| arr.iter().map(|val| val.as_str().unwrap_or_default()).collect::<Vec<_>>()).unwrap_or_default();
                let conf = pkg_info.get("Conflicts").and_then(|val| val.as_array()).map(|arr| arr.iter().map(|val| val.as_str().unwrap_or_default()).collect::<Vec<_>>()).unwrap_or_default();
		let maint = pkg_info.get("Maintainer").and_then(|val| val.as_str()).unwrap_or_default();
	    message.reply(InputMessage::html(format!("<b>Name</b>: <code>{}</code>
<b>Version</b>: <code>{}</code>
<b>Description</b>: {}
<b>URL</b>: {}
<b>Groups</b>: {:?}
<b>Licenses</b>: {:?}
<b>Provides</b>: {:?}
<b>Depends On</b>: {:?}
<b>Make Deps</b>: {:?}
<b>Check Deps</b>: {:?}
<b>Optional Deps</b>: {:?}
<b>Conflicts With</b>: {:?}
<b>Maintainer</b>: {}
", name, ver, desc, url, grp, lic, prov, dep, mkdep, chkdep, optdep, conf, maint))).await?;
	    } else {
	    message.reply("No package found!").await?;
	    }
	}
    } else {
	message.reply("Give me a package to provide info about!").await?;
    }
    return Ok(());
}
