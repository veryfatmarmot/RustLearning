use anyhow::{Context, Result};
use request_handler;
use std::net::TcpListener;
use utils;

fn main() -> Result<()> {
    const ADDR: &str = "127.0.0.1:7877";
    let listener =
        TcpListener::bind(ADDR).with_context(|| format!("Failed to bind to http://{ADDR}"))?;
    println!("Server listening on http://{ADDR}");

    for stream in listener.incoming() {
        let _scope_timer = utils::ScopeTimeLogger::new("handle_connection scope");

        let stream = stream.context("Failed to accept connection")?;

        if let Err(e) = request_handler::handle_connection(stream) {
            eprintln!("Connection error: {e}");
        }
    }

    Ok(())
}
