use reqwest::{
    blocking::RequestBuilder,
    header::{HeaderMap, HeaderValue},
};

use std::env;

pub struct Client {
    client: reqwest::blocking::Client,
    domain: String,
    pub device_id: String,
}

impl Client {
    pub fn new() -> Self {
        let domain = env::var("HA_DOMAIN").unwrap();
        let device_id = env::var("DEVICE_ID").unwrap();
        let auth_token = format!("Bearer {}", env::var("HA_ACCESS_TOKEN").unwrap());

        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        headers.insert("Authorization", HeaderValue::from_str(&auth_token).unwrap());

        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Client {
            client,
            domain,
            device_id,
        }
    }

    pub fn get(&self, path: &str) -> RequestBuilder {
        self.client.get(format!("https://{}{}", self.domain, path))
    }
}
