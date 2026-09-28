use crate::config::Config;
use anyhow::Result;
use ureq::{Body, Error, http::{Response}};

pub struct Client {
    api_key: String,
    base_url: String,
    pub device_id: String,
}

impl Client {
    pub fn new(config: Config) -> Result<Self> {
        Ok(Client {
            api_key: config.api_key,
            device_id: config.device_id,
            base_url: config.base_url,
        })
    }

    pub fn get(&self, path: &str) -> Result<Response<Body>, Error> {
        ureq::get(format!("{}{}", self.base_url, path))
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .call()
    }
}
