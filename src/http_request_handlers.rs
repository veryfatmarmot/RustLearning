use anyhow::Result;
use lazy_static::lazy_static;
use std::{boxed::Box, collections::HashMap, fs, path::Path};

type Response = Vec<u8>;

pub trait Handler {
    fn handle(&self) -> Result<Response>;
}

type HandlerBoxed = Box<dyn Handler + Send + Sync>;

lazy_static! {
    pub static ref NOT_FOUND_HANDLER: HandlerBoxed = Box::new(NotFoundHandler);
    pub static ref HANDLES: HashMap<&'static str, HandlerBoxed> = {
        let mut map: HashMap<&'static str, HandlerBoxed> = HashMap::new();
        map.insert("/", Box::new(HomeHandler));
        map.insert("/favicon.ico", Box::new(FaviconHandler));
        map
    };
}

/// Returns a 404 Not Found response.
struct NotFoundHandler;
impl Handler for NotFoundHandler {
    fn handle(&self) -> Result<Response> {
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
}

/// Returns the main page response
struct HomeHandler;
impl Handler for HomeHandler {
    fn handle(&self) -> Result<Response> {
        let path = Path::new("resources/hello.html");
        let contents = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to read file {}: {}", path.display(), e);
                return NOT_FOUND_HANDLER.as_ref().handle();
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

/// Returns the favicon response by reading favicon.ico.
struct FaviconHandler;
impl Handler for FaviconHandler {
    fn handle(&self) -> Result<Response> {
        let path = Path::new("resources/favicon.ico");
        let contents = match fs::read(path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to read file {}: {}", path.display(), e);
                return NOT_FOUND_HANDLER.as_ref().handle();
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