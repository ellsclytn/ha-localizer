mod ichnaea;
mod location;
mod server;

use ha_localizer::config::Config;
use std::process;

fn main() {
    let config = match Config::new() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            process::exit(1);
        }
    };

    match server::Server::new(config) {
        Ok(s) => s.listen(),
        Err(e) => {
            eprintln!("Error initalizing server: {}", e);
            process::exit(1);
        }
    };
}
