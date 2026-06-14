//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Download a URL or replied Telegram media to the dl/ folder.

use grammers_client::{
    Client,
    media::Media,
    message::{InputMessage, Message},
};
use reqwest::Url;
use std::path::{Path, PathBuf};
use tokio::fs;

type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

const DL_DIR: &str = "dl";
const DOWNLOAD_FAILED: &str = "<b>Download failed!</b>";
const DOWNLOAD_STARTED: &str = "<b>Downloading file...</b>";
const DOWNLOAD_USAGE: &str = "Reply to a <b>file</b> or give me a <b>proper download link</b>!";

fn filename_from_url(url: &Url) -> String {
    url.path_segments()
        .and_then(|segments| segments.filter(|segment| !segment.is_empty()).last())
        .map(sanitize_filename)
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "download".to_string())
}

fn sanitize_filename(name: &str) -> String {
    name.trim()
        .trim_matches('.')
        .chars()
        .map(|ch| match ch {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0' => '_',
            ch if ch.is_whitespace() => '_',
            _ => ch,
        })
        .collect()
}

fn unique_path(dir: &Path, filename: &str) -> PathBuf {
    let mut path = dir.join(filename);
    if !path.exists() {
        return path;
    }

    let source = Path::new(filename);
    let stem = source
        .file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .unwrap_or("download");
    let extension = source.extension().and_then(|value| value.to_str());

    for index in 1.. {
        let candidate = match extension {
            Some(extension) => format!("{}-{}.{}", stem, index, extension),
            None => format!("{}-{}", stem, index),
        };
        path = dir.join(candidate);
        if !path.exists() {
            return path;
        }
    }

    unreachable!()
}

async fn download_path(
    filename: &str,
) -> std::result::Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    let dir = Path::new(DL_DIR);
    fs::create_dir_all(dir).await?;
    Ok(unique_path(dir, filename))
}

fn filename_from_media(media: &Media) -> String {
    match media {
        Media::Photo(photo) => format!("photo_{}.jpg", photo.id()),
        Media::Document(document) => document
            .name()
            .map(sanitize_filename)
            .filter(|name| !name.is_empty())
            .unwrap_or_else(|| format!("document_{}", document.id())),
        Media::Sticker(sticker) => sticker
            .document
            .name()
            .map(sanitize_filename)
            .filter(|name| !name.is_empty())
            .unwrap_or_else(|| format!("sticker_{}.webp", sticker.document.id())),
        _ => "download".to_string(),
    }
}

async fn download_url(message: &Message, url: Url) -> Result {
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

    let path = download_path(&filename_from_url(&url)).await?;
    fs::write(&path, bytes).await?;
    status
        .edit(InputMessage::new().html(format!(
            "<b>Downloaded:</b> <code>{}</code>",
            path.display()
        )))
        .await?;

    return Ok(());
}

async fn download_reply_media(client: Client, message: &Message, media: Media) -> Result {
    let status = message
        .reply(InputMessage::new().html(DOWNLOAD_STARTED))
        .await?;
    let path = download_path(&filename_from_media(&media)).await?;

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

    if link.is_empty() {
        if let Some(reply) = message.get_reply().await? {
            if let Some(media) = reply.media() {
                download_reply_media(client, message, media).await?;
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

    download_url(message, url).await?;

    return Ok(());
}
