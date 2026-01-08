mod http_request_handlers;

use anyhow::{Context, Result, ensure};
use std::{
    io::{BufReader, prelude::*},
    net::TcpStream,
};

/// Handles a single TCP connection: reads the request, parses it, and sends a response.
pub fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let uri = get_uri(&stream)?;

    // Route to response based on URI
    let handler = http_request_handlers::HANDLES
        .get(uri.as_str())
        .map(|b| b.as_ref())
        .unwrap_or(http_request_handlers::NOT_FOUND_HANDLER.as_ref());
    let response = handler.handle()?;

    // Print response for debugging
    if let Ok(str) = std::str::from_utf8(&response) {
        println!("Response: {str}");
    } else {
        println!("Response: [binary data]");
    }

    // Return the response
    stream
        .write_all(&response)
        .context("Failed to write response")?;
    stream.flush().context("Failed to flush response")?;

    Ok(())
}

/// Reads the HTTP request from the stream and extracts the URI.
pub fn get_uri(stream: &TcpStream) -> Result<String> {
    let buf_reader = BufReader::new(stream);

    // Read request lines, handling I/O errors properly
    let http_request: Vec<String> = buf_reader
        .lines()
        .take_while(|line| match line {
            Ok(l) => !l.is_empty(),
            Err(_) => false, // Stop on error
        })
        .collect::<Result<_, _>>()
        .context("Failed to read request lines")?;

    println!("Request: {http_request:#?}\n\n");

    // Ensure we have at least the first line
    ensure!(!http_request.is_empty(), "Invalid HTTP request: no headers");

    // Parse the first line (e.g., "GET / HTTP/1.1")
    let mut header_first_line_parts = http_request[0].split_whitespace();
    let http_method = header_first_line_parts
        .next()
        .context("Invalid first header line: missing METHOD")?;
    let uri = header_first_line_parts
        .next()
        .context("Invalid first header line: missing URI")?;

    // Basic validation
    ensure!(http_method == "GET", "Only GET method is supported");
    ensure!(uri.starts_with('/'), "Invalid URI: must start with '/'");

    Ok(uri.to_string())
}