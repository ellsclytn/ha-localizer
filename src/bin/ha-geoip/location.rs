use std::rc::Rc;

use crate::{config::Config, home_assistant, ichnaea};
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DeviceLocation {
    latitude: f32,
    longitude: f32,
    gps_accuracy: f32,
}

#[derive(Debug, Deserialize)]
struct LocationResponse {
    attributes: DeviceLocation,
}

pub struct LocationProvider {
    client: home_assistant::Client,
}

impl LocationProvider {
    pub fn new(config: Rc<Config>) -> LocationProvider {
        let client = home_assistant::Client::new(config);

        LocationProvider { client }
    }

    pub fn get_location(&self) -> Result<ichnaea::Response> {
        let path = format!("/api/states/device_tracker.{}", self.client.device_id);
        let response: LocationResponse = self
            .client
            .get(&path)
            .send()
            .context("Location request failed")?
            .json()
            .context("Failed to parse location response from Home Assistant")?;

        Ok(ichnaea::Response {
            accuracy: response.attributes.gps_accuracy,
            location: ichnaea::Location {
                lat: response.attributes.latitude,
                lng: response.attributes.longitude,
            },
        })
    }
}
