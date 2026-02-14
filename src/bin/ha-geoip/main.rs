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

    let server = server::Server::new(config);
    server.listen();
}
