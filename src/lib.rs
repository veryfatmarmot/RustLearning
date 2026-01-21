use anyhow::{Context, Result};
use request_handler;
use std::future::Future;
use tokio::{self, net::TcpListener};

// ===============================================================================================

pub async fn run_server<F>(addr: &str, stop_handle: F) -> Result<()>
where
    F: Future<Output = ()>,
{
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("Failed to bind to http://{addr}"))?;

    println!("Server listening on http://{addr}");

    let mut stop_handle = Box::pin(stop_handle);

    let mut join_set = tokio::task::JoinSet::new();

    loop {
        tokio::select! {
            _ = &mut stop_handle => { break; },
            accept_result = listener.accept() => {
                let (stream, _) = accept_result?;
                let _ = join_set.spawn(async move {
                    if let Err(e) = request_handler::handle_connection(stream).await {
                        eprintln!("Connection handling error: {e}");
                    }
                });
            },
        }

        if join_set.len() > 100 {
            while let Some(res) = join_set.try_join_next() {
                if let Err(e) = res {
                    eprintln!("Connection task error: {e}");
                }
            }
        }
    }

    join_set.join_all().await;

    eprintln!("Server stopped");
    Ok(())
}
