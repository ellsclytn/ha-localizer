use crate::config::Config;
use reqwest::{
    blocking::RequestBuilder,
    header::{HeaderMap, HeaderValue},
};

pub struct Client {
    client: reqwest::blocking::Client,
    base_url: String,
    pub device_id: String,
}

impl Client {
    pub fn new(config: Config) -> Self {
        let auth_token = format!("Bearer {}", config.api_key);

        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        headers.insert("Authorization", HeaderValue::from_str(&auth_token).unwrap());

        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Client {
            client,
            device_id: config.device_id,
            base_url: config.base_url,
        }
    }

    pub fn get(&self, path: &str) -> RequestBuilder {
        self.client.get(format!("{}{}", self.base_url, path))
    }
}
