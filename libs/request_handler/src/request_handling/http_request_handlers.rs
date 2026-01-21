use anyhow::Result;
use async_trait::async_trait;
use lazy_static::lazy_static;
use regex::Regex;
use std::{boxed::Box};
use tokio::fs;
use utils;

type Response = Vec<u8>;

#[async_trait]
pub trait Handler {
    async fn handle(&self) -> Result<Response>;
}

type BoxedHandler = Box<dyn Handler + Send + Sync>;

lazy_static! {
    pub static ref NOT_FOUND_HANDLER: BoxedHandler = Box::new(HandlerNotFound);
    pub static ref HANDLES: Vec<(Regex, BoxedHandler)> = {
        vec![
            (Regex::new(r"^/$").unwrap(), Box::new(HandlerHome)),
            (
                Regex::new(r"^/favicon\.ico$").unwrap(),
                Box::new(HandlerFavicon),
            ),
            (
                Regex::new(r"^/dbg_long_2s$").unwrap(),
                Box::new(HandlerDbgLong::new(2)),
            ),
        ]
    };
}

trait HtmlResponder {
    async fn respond_with_html(&self, html_path: &str) -> Result<Response> {
        let path = utils::path_from_root(html_path);
        let contents = match fs::read_to_string(&path).await {
            Ok(c) => c,
            Err(e) => {
                eprintln!(
                    "Failed to read file {:?}: {}",
                    utils::path_to_absolute(&path),
                    e
                );
                return NOT_FOUND_HANDLER.as_ref().handle().await;
            }
        };
        let length = contents.len();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
            length, contents
        );
        Ok(response.into_bytes())
    }
}

/// Returns a 404 Not Found response.
struct HandlerNotFound;

#[async_trait]
impl Handler for HandlerNotFound {
    async fn handle(&self) -> Result<Response> {
        let path = utils::path_from_root("resources/404.html");
        let contents = match fs::read_to_string(&path).await {
            Ok(c) => c,
            Err(e) => {
                eprintln!(
                    "Failed to read file {:?}: {}",
                    utils::path_to_absolute(&path),
                    e
                );
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
}

/// Simulates a long processing time before returning an HTML response.
struct HandlerDbgLong {
    delay: u32,
}
impl HandlerDbgLong {
    pub fn new(delay: u32) -> Self {
        Self { delay }
    }
}
impl HtmlResponder for HandlerDbgLong {}

#[async_trait]
impl Handler for HandlerDbgLong {
    async fn handle(&self) -> Result<Response> {
        std::thread::sleep(std::time::Duration::from_secs(self.delay as u64));
        self.respond_with_html("resources/debug.html").await
    }
}

/// Returns the main page response
struct HandlerHome;
impl HtmlResponder for HandlerHome {}

#[async_trait]
impl Handler for HandlerHome {
    async fn handle(&self) -> Result<Response> {
        self.respond_with_html("resources/hello.html").await
    }
}

/// Returns the favicon response by reading favicon.ico.
struct HandlerFavicon;

#[async_trait]
impl Handler for HandlerFavicon {
    async fn handle(&self) -> Result<Response> {
        let path = utils::path_from_root("resources/favicon.ico");
        let contents = match fs::read(&path).await {
            Ok(c) => c,
            Err(e) => {
                eprintln!(
                    "Failed to read file {:?}: {}",
                    utils::path_to_absolute(&path),
                    e
                );
                return NOT_FOUND_HANDLER.as_ref().handle().await;
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
}

// TESTS =========================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handler_home() {
        let handler = HandlerHome;
        let result = handler.handle().await;
        assert!(result.is_ok());
        let response = result.unwrap();
        let response_str = String::from_utf8(response).unwrap();
        assert!(response_str.starts_with("HTTP/1.1 200 OK"));
        assert!(response_str.contains("Content-Type: text/html"));
    }

    #[tokio::test]
    async fn test_handler_not_found() {
        let handler = HandlerNotFound;
        let result = handler.handle().await;
        assert!(result.is_ok());
        let response = result.unwrap();
        let response_str = String::from_utf8(response).unwrap();
        assert!(response_str.starts_with("HTTP/1.1 404 NOT FOUND"));
        assert!(response_str.contains("Content-Type: text/html"));
    }

    #[tokio::test]
    async fn test_handler_favicon() {
        let handler = HandlerFavicon;
        let result = handler.handle().await;
        assert!(result.is_ok());
        let response = result.unwrap();
        let response_str = String::from_utf8_lossy(&response);
        assert!(response_str.starts_with("HTTP/1.1 200 OK"));
        assert!(response_str.contains("Content-Type: image/x-icon"));
    }
}
