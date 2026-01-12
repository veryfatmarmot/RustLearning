use request_handler;
use std::{
    io::{Read, Write},
    net::TcpListener,
    net::TcpStream,
    sync::{Arc, Mutex, OnceLock, Weak, atomic, atomic::AtomicBool},
    thread, time,
};

const HOTS_PORT: u16 = 7877;

// ===============================================================================================
struct TestHostServer {
    handle: Option<thread::JoinHandle<()>>,
    running: Arc<AtomicBool>,
}

impl TestHostServer {
    fn new(port: u16) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let should_be_running = running.clone();

        let handle = thread::spawn(move || TestHostServer::run_loop(should_be_running, port));

        eprintln!("Start requested for host server on port {}", port);

        // Wait for server to start
        thread::sleep(time::Duration::from_millis(100));

        Self {
            handle: Some(handle),
            running,
        }
    }

    fn run_loop(should_be_running: Arc<AtomicBool>, port: u16) {
        eprintln!("Starting host server on port {}", port);

        let addr = format!("127.0.0.1:{}", port);
        match TcpListener::bind(&addr) {
            Ok(listener) => {
                listener
                    .set_nonblocking(true)
                    .expect("can't set a listener non-blocking");

                while should_be_running.load(atomic::Ordering::Acquire) {
                    if let Ok((stream, _)) = listener.accept() {
                        if let Err(e) = request_handler::handle_connection(stream) {
                            eprintln!("Connection error: {e}");
                        }
                    } else {
                        thread::sleep(time::Duration::from_millis(10));
                    }
                }

                eprintln!("Server on port {} is stopping", port);
            }
            Err(e) => eprintln!("Bind failed on {}: {}", addr, e),
        }
    }
}

impl Drop for TestHostServer {
    fn drop(&mut self) {
        eprintln!("Dropping host server");
        if let Some(h) = self.handle.take() {
            self.running.store(false, atomic::Ordering::Release);
            let _ = h.join();
        }
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

fn test_request(request: &str) -> String {
    let mut response = String::new();

    {
        // Ensure host server is started
        let _server = lazy_start_host_server();

        // Connect and send request
        let addr = format!("127.0.0.1:{}", HOTS_PORT);
        if let Ok(mut stream) = TcpStream::connect(&addr) {
            stream.write_all(request.as_bytes()).unwrap();
            let mut buffer = [0; 2048];
            if let Ok(n) = stream.read(&mut buffer) {
                response = String::from_utf8_lossy(&buffer[..n]).to_string();
            }
        } else {
            eprintln!("Connect failed to {}", addr);
        }
    }

    response
}

// ===============================================================================================

#[test]
fn test_server_root() {
    let response = test_request("GET / HTTP/1.1\r\nHost: localhost\r\n\r\n");
    assert!(
        response.starts_with("HTTP/1.1 200 OK"),
        "test_server_root Response was: {response}"
    );
    assert!(response.contains("Content-Type: text/html"));
}

#[test]
fn test_server_favicon() {
    let response = test_request("GET /favicon.ico HTTP/1.1\r\nHost: localhost\r\n\r\n");
    assert!(
        response.starts_with("HTTP/1.1 200 OK"),
        "test_server_favicon Response was: {response}"
    );
    assert!(response.contains("Content-Type: image/x-icon"));
}

#[test]
fn test_server_not_found() {
    let response = test_request("GET /nonexistent HTTP/1.1\r\nHost: localhost\r\n\r\n");
    assert!(response.starts_with("HTTP/1.1 404 NOT FOUND"));
    assert!(response.contains("Content-Type: text/html"));
}

#[test]
fn test_server_dbg_long_response() {
    let start_time = time::Instant::now();
    const EXPECTED_DURATION_SECS: u64 = 2;

    let response = test_request(
        format!(
            "GET /dbg_long_{}s HTTP/1.1\r\nHost: localhost\r\n\r\n",
            EXPECTED_DURATION_SECS
        )
        .as_str(),
    );
    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(response.contains("Content-Type: text/html"));

    let duration = (time::Instant::now() - start_time).as_secs();
    assert!(
        duration >= EXPECTED_DURATION_SECS && duration < EXPECTED_DURATION_SECS + 1,
        "Server long request duration was not {} sec, {duration} instead",
        EXPECTED_DURATION_SECS
    );
}

#[test]
fn test_server_multithreaded() {
    let mut handles = vec![];
    for _ in 0..2 {
        handles.push(thread::spawn(|| {
            test_server_dbg_long_response();
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
