use crate::location;
use anyhow::{Context, Result, bail};
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

pub fn process_streams(listener: TcpListener) {
    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            _ => continue,
        };

        match handle_connection(stream) {
            Ok(_) => {}
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn handle_connection(stream: TcpStream) -> Result<()> {
    let buf_reader = BufReader::new(&stream);

    let request_line = match buf_reader.lines().next() {
        Some(l) => l.context("Failed to read request line")?,
        None => {
            bail!("Request has no lines")
        }
    };

    if request_line == "POST / HTTP/1.1" {
        println!("Processing GeoIP request");
        let location_provider = location::LocationProvider::new();
        let response = location_provider.get_location()?;
        let contents = serde_json::to_string(&response).context("Failed to stringify JSON body")?;

        respond(&stream, "200 OK", Some(&contents))?;
        println!("Processed GeoIP request");
    } else {
        println!("Processing invalid request");
        respond(&stream, "404 Not Found", None)?;
        println!("Processed invalid request");
    };

    Ok(())
}

fn respond(mut stream: &TcpStream, status_code: &str, body: Option<&str>) -> Result<()> {
    let status_line = format!("HTTP/1.1 {status_code}");

    let mut response_lines: Vec<String> = Vec::new();
    let separator = "\r\n";
    response_lines.push(status_line);
    response_lines.push("Content-Type: application/json".to_string());

    match body {
        Some(body) => {
            let length = body.len() + 2;
            response_lines.push(format!("Content-Length: {length}"));
            response_lines.push(separator.to_string());
            response_lines.push(body.to_string());
        }
        _ => {
            response_lines.push(format!("Content-Length: 4"));
            response_lines.push(separator.to_string());
            response_lines.push("{}".to_string());
        }
    }

    let response = response_lines.join(separator);
    stream
        .write_all(response.as_bytes())
        .context("Failed to write stream to client")
}
