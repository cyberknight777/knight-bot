//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

use reqwest;
use serde_json::Value;

pub async fn make_request(url: String) -> Option<Value> {
    let response = reqwest::get(&url).await.ok()?.text().await.ok();

    let response = match response {
        Some(val) => val.trim().to_string(),
        None => return None,
    };
    let response: Value = match serde_json::from_str(&response) {
        Ok(val) => val,
        _ => return None,
    };

    Some(response)
}
