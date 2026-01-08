use anyhow::Result;
use lazy_static::lazy_static;
use std::{boxed::Box, collections::HashMap, fs, path::Path};

type Response = Vec<u8>;

pub trait Handler {
    fn handle(&self) -> Result<Response>;
}

type BoxedHandler = Box<dyn Handler + Send + Sync>;

lazy_static! {
    pub static ref NOT_FOUND_HANDLER: BoxedHandler = Box::new(HandlerNotFound);
    pub static ref HANDLES: HashMap<&'static str, BoxedHandler> = {
        let mut map: HashMap<&'static str, BoxedHandler> = HashMap::new();
        map.insert("/", Box::new(HandlerHome));
        map.insert("/favicon.ico", Box::new(HandlerFavicon));
        map
    };
}

/// Returns a 404 Not Found response.
struct HandlerNotFound;
impl Handler for HandlerNotFound {
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
struct HandlerHome;
impl Handler for HandlerHome {
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
struct HandlerFavicon;
impl Handler for HandlerFavicon {
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