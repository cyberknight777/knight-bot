//!
//! Copyright (C) 2023-2025 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

//! Import I/O libraries from the stdlib.
use std::{error::Error, fs::File, io::Read};

#[derive(serde::Deserialize)]

pub struct Config {
    /// TG App API
    pub api_id: i32,
    pub api_hash: String,
    /// TG Bot Token
    pub bot_token: String,
    /// TG Admin User ID
    pub admin_id: i64,
}

impl Config {
    pub fn read() -> Result<Self, Box<dyn Error>> {
        let mut str = String::new();
        File::open("./config.toml")?.read_to_string(&mut str)?;
        Ok(toml::from_str(&str)?)
    }
}
