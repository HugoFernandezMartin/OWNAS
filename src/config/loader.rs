use crate::config::{Config};

use std::fs;
use anyhow::{Context};

pub fn load_config(config_path: &str) -> Result<Config, anyhow::Error> {
    let contents = fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read config file at {}", config_path))?;

    let config: Config = serde_json::from_str(&contents)
        .with_context(|| format!("Failed to parse JSON from {}", config_path))?;

    Ok(config)
}
