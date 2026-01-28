use axum::{Json, Router, http::StatusCode, routing::get};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> (StatusCode, Json<LocationResponse>) {
    let response = LocationResponse {
        message: "Hello, World!".to_string(),
    };

    (StatusCode::OK, Json(response))
}

#[derive(Serialize)]
struct LocationResponse {
    message: String,
}
