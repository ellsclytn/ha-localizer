use chrono::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeviceLocation {
    latitude: f32,
    longitude: f32,
    altitude: f32,
}

pub fn from_location(location: DeviceLocation) -> String {
    let lat_degrees = location.latitude.trunc().abs();
    let lat_minutes = location.latitude.fract().abs() * 60.0;
    let lat_hemisphere = if location.latitude >= 0.0 { "N" } else { "S" };

    let lng_degrees = location.longitude.trunc().abs();
    let lng_minutes = location.longitude.fract().abs() * 60.0;
    let lng_hemisphere = if location.longitude >= 0.0 { "E" } else { "W" };

    let now = chrono::offset::Utc::now();

    let content = format!(
        "GPGGA,{:02}{:02}{:02},{:02}{:.7},{lat_hemisphere},{:02}{:.7},{lng_hemisphere},1,,,{:.3},M,,,,,",
        now.hour(),
        now.minute(),
        now.second(),
        lat_degrees,
        lat_minutes,
        lng_degrees,
        lng_minutes,
        location.altitude
    );

    let mut checksum = 0;
    for c in content.as_bytes() {
        checksum = checksum ^ c;
    }
    // TODO: Checksum should be hex, but geoclue seems to ignore it anyway

    let payload = format!("${content}*{checksum}");

    payload
}
