use std::{
    sync::{Arc, Mutex, OnceLock, Weak},
    thread, time,
};
use tokio::{
    self,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    task::JoinSet,
};

const HOTS_PORT: u16 = 7877;

// ===============================================================================================
struct TestHostServer {
    async_runtime: Option<tokio::runtime::Runtime>,
    shutdown_tx: tokio::sync::watch::Sender<bool>,
}

impl TestHostServer {
    fn new(port: u16) -> Self {
        let async_runtime = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
        let (shutdown_tx, mut shutdown_rx) = tokio::sync::watch::channel(false);

        let addr = format!("127.0.0.1:{}", port);
        async_runtime.spawn(async move {
            let shutdown_handle = async move {
                let _ = shutdown_rx.changed().await;
            };

            if let Err(e) = simple_http_server::run_server(&addr, shutdown_handle).await {
                eprintln!("Host server error: {}", e);
            }
        });

        eprintln!("Start requested for host server on port {}", port);

        // Wait for server to start
        thread::sleep(time::Duration::from_millis(100));

        Self {
            async_runtime: Some(async_runtime),
            shutdown_tx,
        }
    }
}

impl Drop for TestHostServer {
    fn drop(&mut self) {
        eprintln!("Dropping host server");

        if let Err(e) = self.shutdown_tx.send(true) {
            eprintln!("Failed to send shutdown signal to host server: {}", e);
        }

        // Give it time to shutdown gracefully
        thread::sleep(time::Duration::from_millis(50));

        if let Some(async_runtime) = self.async_runtime.take() {            
            // Drop the runtime in a blocking thread to avoid panic
            std::thread::spawn(move || {
                async_runtime.shutdown_timeout(time::Duration::from_secs(5));
            })
            .join()
            .expect("Failed to join async runtime shutdown thread");
        }

        eprintln!("Host server dropped");
    }
}

fn lazy_start_host_server() -> Arc<TestHostServer> {
    static HOST_SERVER: OnceLock<Mutex<Weak<TestHostServer>>> = OnceLock::new();

    let m = HOST_SERVER.get_or_init(|| Mutex::new(Weak::new()));

    let mut server_weak = m.lock().unwrap();

    match server_weak.upgrade() {
        Some(server) => server,
        None => {
            let server = Arc::new(TestHostServer::new(HOTS_PORT));
            *server_weak = Arc::downgrade(&server);
            server
        }
    }
}

// ===============================================================================================

async fn test_request(request: &str) -> String {
    let mut response = String::new();

    {
        // Ensure host server is started
        let _server = lazy_start_host_server();

        // Connect and send request
        let addr = format!("127.0.0.1:{}", HOTS_PORT);
        if let Ok(mut stream) = TcpStream::connect(&addr).await {
            stream.write_all(request.as_bytes()).await.unwrap();
            stream.flush().await.unwrap();
            let mut buffer = [0; 2048];
            if let Ok(n) = stream.read(&mut buffer).await {
                response = String::from_utf8_lossy(&buffer[..n]).to_string();
            }
        } else {
            eprintln!("Connect failed to {}", addr);
        }
    }

    response
}

// ===============================================================================================

#[tokio::test]
async fn test_server_root() {
    let response = test_request("GET / HTTP/1.1\r\nHost: localhost\r\n\r\n").await;
    assert!(
        response.starts_with("HTTP/1.1 200 OK"),
        "test_server_root Response was: {response}"
    );
    assert!(response.contains("Content-Type: text/html"));
}

#[tokio::test]
async fn test_server_favicon() {
    let response = test_request("GET /favicon.ico HTTP/1.1\r\nHost: localhost\r\n\r\n").await;
    assert!(
        response.starts_with("HTTP/1.1 200 OK"),
        "test_server_favicon Response was: {response}"
    );
    assert!(response.contains("Content-Type: image/x-icon"));
}

#[tokio::test]
async fn test_server_not_found() {
    let response = test_request("GET /nonexistent HTTP/1.1\r\nHost: localhost\r\n\r\n").await;
    assert!(response.starts_with("HTTP/1.1 404 NOT FOUND"));
    assert!(response.contains("Content-Type: text/html"));
}

#[tokio::test]
async fn test_server_dbg_long_response() {
    server_dbg_long_response(None).await;
}

async fn server_dbg_long_response(request_id: Option<u32>) {
    let start_time = time::Instant::now();
    const EXPECTED_DURATION_SECS: u64 = 2;

    let response = test_request(
        format!(
            "GET /dbg_long_{}s HTTP/1.1\r\nHost: localhost\r\n\r\n",
            EXPECTED_DURATION_SECS
        )
        .as_str(),
    )
    .await;

    let id_str = request_id.map_or(String::new(), |id| format!("#{id} "));
    assert!(
        response.starts_with("HTTP/1.1 200 OK"),
        "Server long request {id_str}response was: '{response}'"
    );
    assert!(response.contains("Content-Type: text/html"));

    let duration = (time::Instant::now() - start_time).as_secs();
    assert!(
        duration >= EXPECTED_DURATION_SECS && duration < EXPECTED_DURATION_SECS + 2,
        "Server long request {id_str}duration was not {} sec, {duration} instead",
        EXPECTED_DURATION_SECS
    );
}

#[tokio::test]
async fn test_server_multithreaded() {
    let mut join_set = JoinSet::new();
    for id in 0..10000 {
        let _ = join_set.spawn(async move {
            server_dbg_long_response(Some(id)).await;
        });
    }

    join_set.join_all().await;
}
