//!
//! Copyright (C) 2023-2025 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

use tokio::runtime;

mod cfg;
mod init;
mod plugins;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result {
    pretty_env_logger::init();

    log::info!("Knight-Bot v{} is initializing...", VERSION);
    runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(init::async_main())
}
