use anyhow::Result;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub api_key: String,
    pub base_url: String,
    pub device_id: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Result<Self> {
        let config_str: String = fs::read_to_string("/etc/ha-localizer.toml")?;
        let config: Config = toml::from_str(&config_str)?;

        Ok(config)
    }
}
