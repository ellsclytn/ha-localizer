use crate::{
    config::Config,
    location::{self, LocationProvider},
};
use anyhow::{Context, Result};
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    process,
};

// It's unexpected for the first line of a request to be above this size, so we might as well keep
// the initial allocation small.
const MAX_REQUEST_LINE_BYTES: usize = 128;

pub struct Server {
    port: u16,
    location_provider: LocationProvider,
}

impl Server {
    pub fn new(config: Config) -> Server {
        let port = config.port;
        let location_provider = location::LocationProvider::new(config);

        Server {
            port,
            location_provider,
        }
    }

    pub fn listen(self: &Server) {
        let listener_address = format!("127.0.0.1:{}", self.port);
        let listener = match TcpListener::bind(&listener_address) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Failed to bind to {}: {}", listener_address, e);
                process::exit(1);
            }
        };

        println!("Listening on {listener_address}");

        self.process_streams(listener);
    }

    fn process_streams(self: &Server, listener: TcpListener) {
        for stream in listener.incoming() {
            let stream = match stream {
                Ok(s) => s,
                _ => continue,
            };

            match self.handle_connection(stream) {
                Ok(_) => {}
                Err(e) => eprintln!("{}", e),
            }
        }
    }

    fn handle_connection(self: &Server, stream: TcpStream) -> Result<()> {
        let mut buf_reader = BufReader::with_capacity(MAX_REQUEST_LINE_BYTES, &stream);
        let mut request_line = String::with_capacity(MAX_REQUEST_LINE_BYTES);

        buf_reader
            .read_line(&mut request_line)
            .context("Failed to read request line")?;

        let request_line = request_line.trim();

        if request_line == "POST / HTTP/1.1" {
            println!("Processing GeoIP request");
            let response = self.location_provider.get_location()?;
            let contents =
                serde_json::to_string(&response).context("Failed to stringify JSON body")?;

            self.respond(&stream, "200 OK", Some(&contents))?;
            println!("Processed GeoIP request");
        } else {
            println!("Processing invalid request");
            self.respond(&stream, "404 Not Found", None)?;
            println!("Processed invalid request");
        };

        Ok(())
    }

    fn respond(
        self: &Server,
        mut stream: &TcpStream,
        status_code: &str,
        body: Option<&str>,
    ) -> Result<()> {
        write!(stream, "HTTP/1.1 {}\r\n", status_code)?;
        write!(stream, "Content-Type: application/json\r\n")?;

        match body {
            Some(body) => {
                let length = body.len();
                write!(stream, "Content-Length: {length}\r\n\r\n")?;
                write!(stream, "{body}")?;
            }
            _ => {
                write!(stream, "Content-Length: 2\r\n\r\n{{}}")?;
            }
        };

        Ok(())
    }
}
