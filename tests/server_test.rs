use std::{
    io::{Read, Write},
    net::TcpStream,
    thread, time,
};

fn test_request(port: u16, request: &str) -> String {
    // Start server in a thread on the given port
    let handle = thread::spawn(move || {
        use request_handler;
        use std::net::TcpListener;

        let addr = format!("127.0.0.1:{}", port);
        match TcpListener::bind(&addr) {
            Ok(listener) => {
                if let Ok((stream, _)) = listener.accept() {
                    let _ = request_handler::handle_connection(stream);
                }
            }
            Err(e) => eprintln!("Bind failed on {}: {}", addr, e),
        }
    });

    // Wait for server to start
    thread::sleep(time::Duration::from_millis(100));

    // Connect and send request
    let addr = format!("127.0.0.1:{}", port);
    let mut response = String::new();
    if let Ok(mut stream) = TcpStream::connect(&addr) {
        stream.write_all(request.as_bytes()).unwrap();
        let mut buffer = [0; 2048];
        if let Ok(n) = stream.read(&mut buffer) {
            response = String::from_utf8_lossy(&buffer[..n]).to_string();
        }
    } else {
        eprintln!("Connect failed to {}", addr);
    }

    // Clean up
    let _ = handle.join();
    response
}

#[test]
fn test_server_root() {
    let response = test_request(7879, "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n");
    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(response.contains("Content-Type: text/html"));
}

#[test]
fn test_server_favicon() {
    let response = test_request(7880, "GET /favicon.ico HTTP/1.1\r\nHost: localhost\r\n\r\n");
    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(response.contains("Content-Type: image/x-icon"));
}

#[test]
fn test_server_not_found() {
    let response = test_request(7881, "GET /nonexistent HTTP/1.1\r\nHost: localhost\r\n\r\n");
    assert!(response.starts_with("HTTP/1.1 404 NOT FOUND"));
    assert!(response.contains("Content-Type: text/html"));
}

#[test]
fn test_server_dbg_long_response() {
    let start_time = time::Instant::now();

    let response = test_request(7882, "GET /dbg_long_5s HTTP/1.1\r\nHost: localhost\r\n\r\n");
    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(response.contains("Content-Type: text/html"));
    assert!(response.contains("Test OK"));

    let duration = (time::Instant::now() - start_time).as_secs();
    assert!(
        duration >= 5 && duration < 6,
        "Server long request duration was not 5 sec, {duration} instead"
    );
}
