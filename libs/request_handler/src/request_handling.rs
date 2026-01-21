mod http_request_handlers;

use anyhow::{Context, Result, ensure};
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

static REQUEST_COUNTER: AtomicU64 = AtomicU64::new(1);

// =================================================================================================

/// Handles a single TCP connection: reads the request, parses it, and sends a response.
pub async fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let request_number = REQUEST_COUNTER.fetch_add(1, Ordering::Relaxed);

    let uri = get_uri(&mut stream).await?;

    println!("\n#{request_number} Request: '{uri}'");

    // Route to response based on URI
    let handler = http_request_handlers::HANDLES
        .iter()
        .find(|(re, _)| re.is_match(uri.as_str()))
        .map(|(_, h)| h.as_ref())
        .unwrap_or(http_request_handlers::NOT_FOUND_HANDLER.as_ref());
    let response = handler.handle().await?;

    // Print response for debugging
    if let Ok(utf8_response) = std::str::from_utf8(&response) {
        println!("#{request_number} Response: '{}...'", utf8_response.lines().next().unwrap_or(""));
    } else {
        println!("#{request_number} Response: [binary data]");
    }

    // Return the response
    stream
        .write_all(&response)
        .await
        .context("Failed to write response")?;
    stream.flush().await.context("Failed to flush response")?;

    Ok(())
}

/// Reads the HTTP request from the stream and extracts the URI.
async fn get_uri(stream: &mut TcpStream) -> Result<String> {
    let mut http_request = String::new();

    let mut buf_reader = BufReader::new(stream);
    buf_reader
        .read_line(&mut http_request)
        .await
        .context("Failed to read the first line of the request")?;

    // Ensure we have at least the first line
    ensure!(!http_request.is_empty(), "Invalid HTTP request: no headers");

    // Parse the first line (e.g., "GET / HTTP/1.1")
    let mut header_first_line_parts = http_request.split_whitespace();
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
