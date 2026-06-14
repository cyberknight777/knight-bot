//!
//! Copyright (C) 2023-2026 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

// Description: Displays this text.

use grammers_client::message::Message;
use std::fs;
use std::io::{self, BufRead};
use std::result::Result;

struct CommandInfo {
    name: String,
    description: String,
}

const ADMIN_COMMANDS: &[&str] = &["dl", "mot", "sh", "ul"];

pub async fn knightcmd_help(message: &Message) -> Result<(), Box<dyn std::error::Error>> {
    let mut commands = Vec::new();

    let plugin_dir = "src/plugins";
    for entry in fs::read_dir(plugin_dir)? {
        if let Ok(entry) = entry {
            if let Some(filename) = entry.file_name().to_str() {
                let command_name = filename.trim_end_matches(".rs");
                if filename.ends_with(".rs")
                    && filename != "mod.rs"
                    && filename != "req.rs"
                    && !ADMIN_COMMANDS.contains(&command_name)
                {
                    let command_name = filename.trim_end_matches(".rs").to_string();
                    let description = get_command_description(&command_name, plugin_dir)?;

                    commands.push(CommandInfo {
                        name: command_name,
                        description,
                    });
                }
            }
        }
    }

    commands.sort_by(|a, b| a.name.cmp(&b.name));

    let mut help_msg = "Hello There!, I am a bot made by cyberknight777 in Rust based on gramme.rs.\nHere's a list of my commands (sorted alphabetically):\n".to_owned();
    for command in commands {
        help_msg.push_str(&format!(
            "/{name} - {description}\n",
            name = command.name,
            description = command.description
        ));
    }

    help_msg.push_str("\nAdmin commands:\n");
    for command in ADMIN_COMMANDS {
        help_msg.push_str(&format!(
            "k.{name} - {description}\n",
            name = command,
            description = get_command_description(command, plugin_dir)?
        ));
    }

    message.reply(help_msg).await?;

    Ok(())
}

fn get_command_description(command_name: &str, plugin_dir: &str) -> Result<String, io::Error> {
    let file_path = format!("{}/{}.rs", plugin_dir, command_name);
    let file = fs::File::open(file_path)?;

    let reader = io::BufReader::new(file);
    let mut description = String::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with("// Description:") {
                description = line
                    .trim_start_matches("// Description:")
                    .trim()
                    .to_string();
                break;
            }
        }
    }

    Ok(description)
}
