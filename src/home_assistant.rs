use crate::config::Config;
use reqwest::{
    blocking::RequestBuilder,
    header::{HeaderMap, HeaderValue},
};
use std::rc::Rc;

pub struct Client {
    client: reqwest::blocking::Client,
    config: Rc<Config>,
    pub device_id: String,
}

impl Client {
    pub fn new(config: Rc<Config>) -> Self {
        let config = config.clone();
        let device_id = config.device_id.clone();
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
            config,
            device_id,
        }
    }

    pub fn get(&self, path: &str) -> RequestBuilder {
        self.client.get(format!("{}{}", self.config.base_url, path))
    }
}
