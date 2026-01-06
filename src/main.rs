use anyhow::{Context, Result, ensure};
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    path::Path,
};

fn main() -> Result<()> {
    const ADDR: &str = "127.0.0.1:7877";
    let listener =
        TcpListener::bind(ADDR).with_context(|| format!("Failed to bind to http://{ADDR}"))?;
    println!("Server listening on http://{ADDR}");

    for stream in listener.incoming() {
        let stream = stream.context("Failed to accept connection")?;

        if let Err(e) = handle_connection(stream) {
            eprintln!("Connection error: {e}");
        }
    }

    Ok(())
}

/// Handles a single TCP connection: reads the request, parses it, and sends a response.
fn handle_connection(stream: TcpStream) -> Result<()> {
    let uri = get_uri(&stream)?;

    // Route to response based on URI
    let response = match &uri as &str {
        "/" => get_page_response(),
        "/favicon.ico" => get_favicon_response(),
        _ => get_404_response(),
    }?;

    // Print response for debugging
    if let Ok(str) = std::str::from_utf8(&response) {
        println!("Response: {str}");
    } else {
        println!("Response: [binary data]");
    }

    // Return the response
    let mut stream = stream; // Reborrow as mutable for writing
    stream
        .write_all(&response)
        .context("Failed to write response")?;
    stream.flush().context("Failed to flush response")?;

    Ok(())
}

/// Reads the HTTP request from the stream and extracts the URI.
fn get_uri(stream: &TcpStream) -> Result<String> {
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

    println!("Request: {http_request:#?}/n/n");

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

/// Returns a 404 Not Found response.
fn get_404_response() -> Result<Vec<u8>> {
    let path = Path::new("resources/404.html");
    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read file {}: {}", path.display(), e);
            return Ok(
                "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/plain\r\n\r\n404 Not Found"
                    .as_bytes()
                    .to_vec(),
            );
        }
    };

    let length = contents.len();
    let response = format!(
        "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        length, contents
    );
    Ok(response.into_bytes())
}

/// Returns the main page response by reading hello.html.
fn get_page_response() -> Result<Vec<u8>> {
    let path = Path::new("resources/hello.html");
    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read file {}: {}", path.display(), e);
            return get_404_response();
        }
    };
    let length = contents.len();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        length, contents
    );
    Ok(response.into_bytes())
}

/// Returns the favicon response by reading favicon.ico.
fn get_favicon_response() -> Result<Vec<u8>> {
    let path = Path::new("resources/favicon.ico");
    let contents = match fs::read(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read file {}: {}", path.display(), e);
            return get_404_response();
        }
    };
    let length = contents.len();
    let header = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: image/x-icon\r\nContent-Length: {}\r\n\r\n",
        length
    );
    let mut response = header.into_bytes();
    response.extend(contents);
    Ok(response)
}
