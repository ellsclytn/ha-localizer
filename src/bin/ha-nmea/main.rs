#[path = "../../home_assistant.rs"]
mod home_assistant;
mod nmea;

use nmea::DeviceLocation;
use serde::Deserialize;
use std::fs::{create_dir_all, exists, remove_file};
use std::io::Write;
use std::os::unix::fs::{PermissionsExt, chown};
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;

const SOCKET_PATH: &str = "/run/geoclue/nmea.sock";
const GEOCLUE_UID: u32 = 965;

fn handle_client(mut stream: UnixStream) {
    println!("Processing");
    let location_provider = LocationProvider::new();
    let loc = location_provider.get_location();
    let payload = nmea::from_location(loc);

    let res = stream.write_all(payload.as_bytes());

    match res {
        Err(e) => {
            eprintln!("Error: {:#?}", e)
        }
        _ => {
            println!("Published: {payload}")
        }
    }
}

fn main() -> std::io::Result<()> {
    create_dir_all("/run/geoclue")?;

    let socket_exists = exists(SOCKET_PATH)?;
    if socket_exists {
        remove_file(SOCKET_PATH)?;
    }
    let listener = UnixListener::bind(SOCKET_PATH)?;
    chown(SOCKET_PATH, Some(GEOCLUE_UID), Some(GEOCLUE_UID))?;
    let mut permissions = std::fs::metadata(SOCKET_PATH)?.permissions();
    permissions.set_mode(0o660);

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                /* connection succeeded */
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                /* connection failed */
                break;
            }
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
struct LocationResponse {
    attributes: DeviceLocation,
}

struct LocationProvider {
    client: home_assistant::Client,
}

impl LocationProvider {
    pub fn new() -> LocationProvider {
        let client = home_assistant::Client::new();

        LocationProvider { client }
    }

    pub fn get_location(&self) -> DeviceLocation {
        let path = format!("/api/states/device_tracker.{}", self.client.device_id);
        let response: LocationResponse = self.client.get(&path).send().unwrap().json().unwrap();

        response.attributes
    }
}
