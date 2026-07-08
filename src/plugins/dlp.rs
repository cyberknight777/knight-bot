//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

use grammers_client::media::Media;
use std::path::{Path, PathBuf};
use tokio::fs;

pub struct DownloadDirectory {
    dir: PathBuf,
}

pub fn sanitize_filename(name: &str) -> String {
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

impl DownloadDirectory {
    pub fn new(dir: impl AsRef<Path>) -> Self {
        Self {
            dir: dir.as_ref().to_path_buf(),
        }
    }
    pub async fn download_path(
        &self,
        filename: &str,
    ) -> std::result::Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
        fs::create_dir_all(&self.dir).await?;
        Ok(unique_path(&self.dir, filename))
    }
}

pub fn filename_from_document(media: &Media) -> String {
    match media {
        Media::Document(document) => document
            .name()
            .map(sanitize_filename)
            .filter(|name| !name.is_empty())
            .unwrap_or_else(|| format!("{}", document.id())),
        _ => "download".to_string(),
    }
}

pub fn filename_from_media(media: &Media) -> String {
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
