use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::env;

pub struct Client {
    client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct LocationResponse {
    attributes: DeviceLocation,
}

#[derive(Debug, Deserialize)]
struct DeviceLocation {
    latitude: f32,
    longitude: f32,
    gps_accuracy: f32,
}

#[derive(Debug, Serialize)]
pub struct IchnaeaResponse {
    pub location: Location,
    pub accuracy: f32,
}

#[derive(Debug, Serialize)]
pub struct Location {
    pub lat: f32,
    pub lng: f32,
}

impl Client {
    pub fn new() -> Self {
        let auth_token = format!("Bearer {}", env::var("HA_ACCESS_TOKEN").unwrap());

        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        headers.insert("Authorization", HeaderValue::from_str(&auth_token).unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Client { client }
    }

    pub async fn get_location(&self) -> IchnaeaResponse {
        let url = format!(
            "https://{}/api/states/device_tracker.{}",
            env::var("HA_DOMAIN").unwrap(),
            env::var("DEVICE_ID").unwrap()
        );

        let location: LocationResponse = self
            .client
            .get(url)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let response = IchnaeaResponse {
            location: Location {
                lat: location.attributes.latitude,
                lng: location.attributes.longitude,
            },
            accuracy: location.attributes.gps_accuracy,
        };

        response
    }
}
