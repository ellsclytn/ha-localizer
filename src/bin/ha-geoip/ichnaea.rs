use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub lat: f32,
    pub lng: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub accuracy: f32,
    pub location: Location,
}
