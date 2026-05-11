//!
//! Copyright (C) 2023-2025 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

use grammers_client::{
    Client,
    message::{InputMessage, Message},
};
use std::{
    env,
    path::{Path, PathBuf},
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

fn resolve_upload_path(path: &str) -> std::io::Result<PathBuf> {
    let path = Path::new(path);
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    };

    path.canonicalize()
}

pub async fn knightcmd_ul(client: Client, message: &Message, path: String) -> Result {
    let path = path.trim();

    if path.is_empty() {
        message
            .reply(InputMessage::new().html("Give me a <b>proper file path</b> to upload!"))
            .await?;
        return Ok(());
    }

    let file_path = match resolve_upload_path(path) {
        Ok(path) => path,
        Err(_) => {
            message
                .reply(InputMessage::new().html("<b>File not found!</b>"))
                .await?;
            return Ok(());
        }
    };

    if !file_path.is_file() {
        message
            .reply(InputMessage::new().html("<b>File not found!</b>"))
            .await?;
        return Ok(());
    }

    let status = message
        .reply(InputMessage::new().html("<b>Uploading file...</b>"))
        .await?;

    match client.upload_file(file_path).await {
        Ok(file) => {
            status.delete().await?;
            message
                .reply(InputMessage::new().text("").file(file))
                .await?;
        }
        Err(_) => {
            status
                .edit(InputMessage::new().html("<b>Upload failed!</b>"))
                .await?;
        }
    }

    return Ok(());
}
