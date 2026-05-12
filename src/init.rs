//!
//! Copyright (C) 2023-2025 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

use crate::{cfg, plugins};
use grammers_client::{
    Client, SenderPool, client::UpdatesConfiguration, session::storages::SqliteSession,
};
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
    let session = Arc::new(SqliteSession::open(SESSION_FILE).await?);
    let SenderPool {
        runner,
        updates,
        handle,
    } = SenderPool::new(Arc::clone(&session), api_id);
    let client = Client::new(handle.clone());
    let pool_task = tokio::spawn(runner.run());
    log::info!("Connected!");

    if !client.is_authorized().await? {
        log::info!("Signing in...");
        client.bot_sign_in(&token, &api_hash).await?;
        log::info!("Signed in!");
    }

    let me = client.get_me().await?;
    let bot_username = me.username().unwrap_or("").to_string();

    log::info!("Waiting for messages...");

    let mut updates = client
        .stream_updates(updates, UpdatesConfiguration::default())
        .await;

    loop {
        let update = tokio::select! {
            _ = tokio::signal::ctrl_c() => break,
            update = updates.next() => update?,
        };

        let handle = client.clone();
        let bot_username = bot_username.clone();
        task::spawn(async move {
            match plugins::handle_update(handle, update, &bot_username).await {
                Ok(_) => {}
                Err(e) => log::error!("Error handling updates!: {}", e),
            }
        });
    }

    updates.sync_update_state().await;
    handle.quit();
    let _ = pool_task.await;
    Ok(())
}
