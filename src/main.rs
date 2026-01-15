use anyhow::Context;
use simple_http_server::run_server;
use std::{
    sync::{
        Arc,
        atomic::{self, AtomicBool},
    },
    thread,
};

fn main() -> anyhow::Result<()> {
    let running = Arc::new(AtomicBool::new(true));

    let should_be_running = running.clone();
    let thread_handle = thread::Builder::new()
        .name("the_server".to_string())
        .spawn(move || run_server("127.0.0.1:7877", should_be_running))?;

    println!("enter 'q' to stop the server");
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .context("Failed to read from stdin")?;
        if input.trim() == "q" {
            println!("Start stopping the server...");
            break;
        }
    }

    running.store(false, atomic::Ordering::Release);
    thread_handle
        .join()
        .map_err(|_| anyhow::anyhow!("Server thread panicked"))?
}
