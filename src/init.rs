//!
//! Copyright (C) 2023-2025 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

use crate::{cfg, plugins};
use grammers_client::{Client, UpdatesConfiguration};
use grammers_mtsender::SenderPool;
use grammers_session::storages::SqliteSession;
use log;
use std::sync::Arc;
use tokio::task::JoinSet;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

const SESSION_FILE: &str = "knight-bot.session";

pub async fn async_main() -> Result {
    let config = Arc::new(cfg::Config::read().expect("cannot read the config"));
    let api_id = config.clone().api_id;
    let api_hash = &config.api_hash;
    let token = &config.bot_token;
    let session = Arc::new(SqliteSession::open(SESSION_FILE)?);
    let pool = SenderPool::new(Arc::clone(&session), api_id);

    log::info!("Connecting to Telegram...");
    let client = Client::new(&pool);
    let SenderPool {
        runner,
        updates,
        handle,
    } = pool;
    let pool_task = tokio::spawn(runner.run());
/*    let client = Client::connect(Config {
        session: Session::load_file_or_create(SESSION_FILE)?,
        api_id,
        api_hash: api_hash.clone(),
        params: InitParams {
            ..Default::default()
        },
    })
    .await?;
*/
    log::info!("Connected!");

    if !client.is_authorized().await? {
        log::info!("Signing in...");
        client.bot_sign_in(&token, &api_hash).await?;
        log::info!("Signed in!");
    }

    log::info!("Waiting for messages...");

    let mut handler_tasks = JoinSet::new();
    let mut updates = client.stream_updates(
        updates,
        UpdatesConfiguration {
            catch_up: true,
            ..Default::default()
        },
    );

/*    while let Some(update) = tokio::select! {
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
     */

    loop {
        while let Some(_) = handler_tasks.try_join_next() {}

        tokio::select! {
	_ = tokio::signal::ctrl_c() => break,
	update = updates.next() => {
	    let update = update?;
	    let handle = client.clone();
	    handler_tasks.spawn(plugins::handle_update(handle, update));
	}
        }
    }

    println!("Saving session file...");
    updates.sync_update_state();

    println!("Gracefully closing connection to notify all pending handlers...");
    handle.quit();
    let _ = pool_task.await;

    println!("Waiting for any slow handlers to finish...");
    while let Some(_) = handler_tasks.join_next().await {}

    Ok(())
}
