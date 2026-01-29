use std::env;

use axum::{Json, Router, http::StatusCode, routing::post};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(root));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> (StatusCode, Json<IchnaeaResponse>) {
    let url = format!(
        "https://{}/api/states/device_tracker.{}",
        env::var("HA_DOMAIN").unwrap(),
        env::var("DEVICE_ID").unwrap()
    );
    let auth_token = format!("Bearer {}", env::var("HA_ACCESS_TOKEN").unwrap());

    let location: HomeAssistantResponse = reqwest::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("Authorization", auth_token)
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

    (StatusCode::OK, Json(response))
}

#[derive(Debug, Deserialize)]
struct HomeAssistantResponse {
    attributes: DeviceLocation,
}

#[derive(Debug, Deserialize)]
struct DeviceLocation {
    latitude: f32,
    longitude: f32,
    gps_accuracy: f32,
}

#[derive(Debug, Serialize)]
struct IchnaeaResponse {
    location: Location,
    accuracy: f32,
}

#[derive(Debug, Serialize)]
struct Location {
    lat: f32,
    lng: f32,
}
