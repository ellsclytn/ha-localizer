mod http;
mod ichnaea;
mod location;

use std::{net::TcpListener, process};

const ADDRESS: &str = "127.0.0.1:51144";

fn main() {
    let listener = match TcpListener::bind(ADDRESS) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to bind to {}: {}", ADDRESS, e);
            process::exit(1);
        }
    };

    http::process_streams(listener);
}
