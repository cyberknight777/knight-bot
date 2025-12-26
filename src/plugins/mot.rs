//!
//! Copyright (C) 2023-2025 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Gets OTA zips from Motorola's OTA server.

use grammers_client::types::{InputMessage, Message};
use html_escape;
use reqwest::Client;
use serde_json::{json, Value};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_mot(
    message: Message,
    kuid: Option<String>,
    kcar: Option<String>,
    ksn: Option<String>,
) -> Result {
    let kuid = match kuid {
        Some(k) if !k.is_empty() => k,
        _ => {
            message
                .reply("Missing GUID! Usage: k.mot <otaSourceSha1> <carrier> [serialnumber]")
                .await?;
            return Ok(());
        }
    };

    let kcar = match kcar {
        Some(c) if !c.is_empty() => c,
        _ => {
            message
                .reply("Missing carrier! Usage: k.mot <otaSourceSha1> <carrier> [serialnumber]")
                .await?;
            return Ok(());
        }
    };

    let ksn = match ksn {
        Some(s) if !s.is_empty() => s,
        _ => "SERIAL_NUMBER_NOT_AVAILABLE".to_string(),
    };

    let body = json!({
        "id": ksn,
        "deviceInfo": { "country": "US", "region": "US" },
        "extraInfo": {
    "carrier": kcar,
    "vitalUpdate": false,
    "otaSourceSha1": kuid
        },
        "triggeredBy": "user"
    });

    let url = format!(
        "https://moto-cds.appspot.com/cds/upgrade/1/check/ctx/ota/key/{}",
        kuid
    );

    let client = Client::new();
    let response: Value = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("User-Agent", "com.motorola.ccc.ota")
        .json(&body)
        .send()
        .await?
        .json()
        .await?;

    if response["proceed"].as_bool().unwrap_or(false) {
        let content = &response["content"];

        let display_version = content["displayVersion"].as_str().unwrap_or("N/A");
        let model = content["model"].as_str().unwrap_or("N/A");
        let release_notes = content["releaseNotes"].as_str().unwrap_or("N/A");
        let ota_target = content["otaTargetSha1"].as_str().unwrap_or("N/A");
        let source_version = content["sourceDisplayVersion"].as_str().unwrap_or("N/A");
        let download_url = &response["contentResources"]
            .get(0)
            .and_then(|r| r.get("url"))
            .and_then(|u| u.as_str())
            .unwrap_or("N/A");
        let fingerprint = &content["streamingData"]["header"]["TARGET_FP"]
            .as_str()
            .unwrap_or("N/A");
        let release_notes = html_escape::encode_text(release_notes);
        let reply = format!(
	"<b>Model:</b> <code>{}</code>\n<b>Version</b>: <code>{}</code>\n<b>Previous Version:</b> <code>{}</code>\n<b>Fingerprint:</b> <code>{}</code>\n<b>Next OTA SHA1:</b> <code>{}</code>\n<b>Changelog:</b> <code><pre>{}</pre></code>\n<b>Download:</b> {}",
	    model, display_version, source_version, fingerprint, ota_target, release_notes, download_url
        );

        message.reply(InputMessage::html(reply)).await?;
    } else {
        message
            .reply(InputMessage::html(format!(
                "<b>No OTA available or bad response!</b>"
            )))
            .await?;
    }
    return Ok(());
}
