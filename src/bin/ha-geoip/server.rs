use crate::{
    config::Config,
    location::{self, LocationProvider},
};
use anyhow::{Context, Result, bail};
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    process,
};

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
        let buf_reader = BufReader::new(&stream);

        let request_line = match buf_reader.lines().next() {
            Some(l) => l.context("Failed to read request line")?,
            None => {
                bail!("Request has no lines")
            }
        };

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
