//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

use crate::{cfg, plugins};
use grammers_client::{Client, Config, InitParams};
use grammers_session::Session;
use log;
use std::sync::Arc;
use tokio::task;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

const SESSION_FILE: &str = "knight-bot.session";

pub async fn async_main() -> Result {
    let config = Arc::new(cfg::Config::read().expect("cannot read the config"));
    let api_id = config.clone().api_id;
    let api_hash = &config.api_hash;
    let token = &config.bot_token;

    log::info!("Connecting to Telegram...");
    let client = Client::connect(Config {
        session: Session::load_file_or_create(SESSION_FILE)?,
        api_id,
        api_hash: api_hash.clone(),
        params: InitParams {
            ..Default::default()
        },
    })
    .await?;
    log::info!("Connected!");

    if !client.is_authorized().await? {
        log::info!("Signing in...");
        client.bot_sign_in(&token, api_id, &api_hash).await?;
        client.session().save_to_file(SESSION_FILE)?;
        log::info!("Signed in!");
    }

    log::info!("Waiting for messages...");

    while let Some(update) = tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(None),
        result = client.next_update() => result,
    }? {
        let handle = client.clone();
        task::spawn(async move {
            match plugins::handle_update(handle, update).await {
                Ok(_) => {}
                Err(e) => log::error!("Error handling updates!: {}", e),
            }
        });
    }

    client.session().save_to_file(SESSION_FILE)?;
    Ok(())
}
