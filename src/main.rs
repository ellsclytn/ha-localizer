mod home_assistant;

use axum::{Json, Router, http::StatusCode, routing::post};
use home_assistant::{Client, IchnaeaResponse};
use std::{env, thread};

#[tokio::main]
async fn main() {
    let port = match env::var("PORT") {
        Ok(port) => port,
        _ => "3000".to_string(),
    };

    thread::spawn(|| {
        println!("Hello, thread!");
    });

    let app = Router::new().route("/", post(root));
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> (StatusCode, Json<IchnaeaResponse>) {
    let ha = Client::new();
    let location = ha.get_location().await;

    (StatusCode::OK, Json(location))
}
