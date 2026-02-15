use anyhow::{Result, bail};
use ha_localizer::{config::Config, home_assistant::Client};
use serde::{Deserialize, Serialize};
use std::{fs, os::unix::fs::symlink, process};

const ETC_LOCALTIME: &str = "/etc/localtime";
const ZONEINFO_PATH: &str = "/usr/share/zoneinfo";

fn main() {
    let provider = TimezoneProvider::new();

    match provider.sync_timezone() {
        Ok(tz) => println!("Timezone set to {tz}"),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    }
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

    pub fn sync_timezone(&self) -> Result<String> {
        let tz = self.get_timezone()?.attributes.time_zone_id;
        self.set_timezone(&tz)?;

        Ok(tz)
    }

    fn get_timezone(&self) -> Result<TimezoneResponse> {
        let path = format!(
            "/api/states/sensor.{}_current_time_zone",
            self.client.device_id
        );
        let response: TimezoneResponse = self.client.get(&path).send()?.json()?;

        Ok(response)
    }

    fn set_timezone(&self, tz: &str) -> Result<()> {
        let original = format!("{ZONEINFO_PATH}/{tz}");
        match fs::exists(&original) {
            Ok(false) => bail!("{tz} does not exist in zoneinfo."),
            Err(e) => bail!(
                "Unable to verify whether {tz} exists. Check the application has permission to read {ZONEINFO_PATH}. {e}"
            ),
            _ => {}
        }

        fs::remove_file(ETC_LOCALTIME)?;
        symlink(original, ETC_LOCALTIME)?;

        Ok(())
    }
}
