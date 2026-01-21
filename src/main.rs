use anyhow::{Result};
use simple_http_server::run_server;
use tokio::{
    io::{AsyncBufReadExt, BufReader, stdin},
};

// ===============================================================================================

#[tokio::main]
async fn main() -> Result<()> {

    run_server("127.0.0.1:7877", handle_input()).await?;

    Ok(())
}

async fn handle_input() {
    println!("enter 'q' to stop the server");
    let mut stdin = BufReader::new(stdin());
    loop {
        let mut input = String::new();
        if let Err(e) = stdin.read_line(&mut input).await {
            eprintln!("Failed to read from stdin: {e}");
        } else if input.trim() == "q" {
            println!("Start stopping the server...");
            break;
        }
    }
}
