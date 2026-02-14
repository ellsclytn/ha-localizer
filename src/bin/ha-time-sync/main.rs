#[path = "../../config.rs"]
mod config;
#[path = "../../home_assistant.rs"]
mod home_assistant;

use crate::config::Config;
use home_assistant::Client;
use serde::{Deserialize, Serialize};
use std::{fs, os::unix::fs::symlink, process};

fn main() {
    let provider = TimezoneProvider::new();

    provider.sync_timezone();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimezoneResponse {
    attributes: DeviceTimezone,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceTimezone {
    time_zone_id: String,
}

struct TimezoneProvider {
    client: Client,
}

impl TimezoneProvider {
    pub fn new() -> Self {
        let config = match Config::new() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to load config: {}", e);
                process::exit(1);
            }
        };
        let client = Client::new(config);

        TimezoneProvider { client }
    }

    pub fn sync_timezone(&self) {
        let tz = self.get_timezone().attributes.time_zone_id;

        self.set_timezone(&tz);
    }

    fn get_timezone(&self) -> TimezoneResponse {
        let path = format!(
            "/api/states/sensor.{}_current_time_zone",
            self.client.device_id
        );

        let response: TimezoneResponse = self.client.get(&path).send().unwrap().json().unwrap();

        response
    }

    fn set_timezone(&self, tz: &str) {
        let original = format!("/usr/share/zoneinfo/{tz}");
        let link = "/etc/localtime";

        fs::remove_file(link).unwrap();

        symlink(original, link).unwrap();

        println!("Timezone set to {tz}");
    }
}
