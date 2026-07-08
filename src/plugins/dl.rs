//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Download a URL or replied Telegram media to the dl/ folder.

use crate::plugins::dlp;
use dlp::DownloadDirectory;
use grammers_client::{
    Client,
    media::Media,
    message::{InputMessage, Message},
};
use reqwest::Url;
use tokio::fs;

type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

const DOWNLOAD_FAILED: &str = "<b>Download failed!</b>";
const DOWNLOAD_STARTED: &str = "<b>Downloading file...</b>";
const DOWNLOAD_USAGE: &str = "Reply to a <b>file</b> or give me a <b>proper download link</b>!";

fn filename_from_url(url: &Url) -> String {
    url.path_segments()
        .and_then(|segments| segments.filter(|segment| !segment.is_empty()).last())
        .map(dlp::sanitize_filename)
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "download".to_string())
}

async fn download_url(message: &Message, url: Url, dir: DownloadDirectory) -> Result {
    let status = message
        .reply(InputMessage::new().html(DOWNLOAD_STARTED))
        .await?;

    let response = match reqwest::get(url.clone()).await {
        Ok(response) => response,
        Err(_) => {
            status
                .edit(InputMessage::new().html(DOWNLOAD_FAILED))
                .await?;
            return Ok(());
        }
    };

    if !response.status().is_success() {
        status
            .edit(
                InputMessage::new().html(format!("{DOWNLOAD_FAILED}\nHTTP {}", response.status())),
            )
            .await?;
        return Ok(());
    }

    let bytes = match response.bytes().await {
        Ok(bytes) => bytes,
        Err(_) => {
            status
                .edit(InputMessage::new().html(DOWNLOAD_FAILED))
                .await?;
            return Ok(());
        }
    };

    let filename = filename_from_url(&url);
    let path = dir.download_path(&filename).await?;
    fs::write(&path, bytes).await?;
    status
        .edit(InputMessage::new().html(format!(
            "<b>Downloaded:</b> <code>{}</code>",
            path.display()
        )))
        .await?;

    return Ok(());
}

async fn download_reply_media(
    client: Client,
    message: &Message,
    media: Media,
    dir: DownloadDirectory,
) -> Result {
    let status = message
        .reply(InputMessage::new().html(DOWNLOAD_STARTED))
        .await?;
    let filename = dlp::filename_from_media(&media);
    let path = dir.download_path(&filename).await?;

    match client.download_media(&media, &path).await {
        Ok(_) => {
            status
                .edit(InputMessage::new().html(format!(
                    "<b>Downloaded:</b> <code>{}</code>",
                    path.display()
                )))
                .await?;
        }
        Err(_) => {
            status
                .edit(InputMessage::new().html(DOWNLOAD_FAILED))
                .await?;
        }
    }

    return Ok(());
}

pub async fn knightcmd_dl(client: Client, message: &Message, link: String) -> Result {
    let link = link.trim();
    let dl_dir = dlp::DownloadDirectory::new("dl");

    if link.is_empty() {
        if let Some(reply) = message.get_reply().await? {
            if let Some(media) = reply.media() {
                download_reply_media(client, message, media, dl_dir).await?;
            } else {
                message
                    .reply(InputMessage::new().html(DOWNLOAD_USAGE))
                    .await?;
            }
        } else {
            message
                .reply(InputMessage::new().html(DOWNLOAD_USAGE))
                .await?;
        }
        return Ok(());
    }

    let url = match Url::parse(link) {
        Ok(url) if matches!(url.scheme(), "http" | "https") => url,
        _ => {
            message
                .reply(InputMessage::new().html("<b>Invalid download link!</b>"))
                .await?;
            return Ok(());
        }
    };

    download_url(message, url, dl_dir).await?;

    return Ok(());
}
