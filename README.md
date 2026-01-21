# Simple HTTP Server

A lightweight, asynchronous HTTP server written in Rust as a learning project. This server demonstrates concurrent connection handling using Tokio and includes custom request handling capabilities.

## Features

- **Asynchronous I/O**: Built with Tokio for efficient concurrent connection handling
- **Connection Limiting**: Supports up to 10,000 concurrent connections
- **Cross-Platform**: Works on both Windows and Unix-based systems
- **Modular Architecture**: Organized into separate libraries for request handling and utilities
- **Graceful Shutdown**: Interactive console control with graceful server termination

## Project Structure

```
simple_http_server/
├── src/
│   ├── main.rs           # Entry point and server initialization
│   └── lib.rs            # Core server logic
├── libs/
│   ├── request_handler/  # HTTP request handling library
│   └── utils/            # Utility functions (thread pool, logging)
├── resources/            # HTML files and static resources
│   ├── hello.html
│   ├── 404.html
│   └── debug.html
├── tests/                # Integration tests
└── Cargo.toml           # Project configuration
```

## Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/simple_http_server.git
cd simple_http_server
```

2. Build the project:
```bash
cargo build --release
```

## Usage

### Running the Server

Start the server with:
```bash
cargo run
```

The server will start listening on `http://127.0.0.1:7877` by default.

### Stopping the Server

Type `q` in the terminal and press Enter to gracefully stop the server.

### Testing

Run the test suite:
```bash
cargo test --workspace
```

## Configuration

The server binds to `127.0.0.1:7877` by default. To change the address or port, modify the following line in `src/main.rs`:

```rust
run_server("127.0.0.1:7877", handle_input()).await?;
```

## Dependencies

- **tokio** (1.49.0): Asynchronous runtime with full features
- **anyhow** (1.0.100): Error handling
- **request_handler**: Custom HTTP request handler (local library)
- **utils**: Utility functions (local library)

### Platform-Specific Dependencies

- **Unix**: `libc` (0.2)
- **Windows**: `windows-sys` (0.52)

## Development

### Building for Development

```bash
cargo build
```

### Running in Debug Mode

```bash
cargo run
```

### Generating Documentation

```bash
cargo doc --open
```

## Workspace Structure

This project uses Cargo workspaces with the following members:
- `libs/request_handler`: HTTP request parsing and handling
- `libs/utils`: Shared utilities (thread pool, scope time logger)

## License

This project is licensed under the MIT License.

## Learning Goals

This project was created as a Rust learning exercise to explore:
- Asynchronous programming with Tokio
- TCP socket handling
- HTTP protocol basics
- Rust's module system and workspace organization
- Concurrent programming patterns
- Error handling with `anyhow`

## Acknowledgments

Built as part of learning Rust programming language. Special thanks to the Rust community and the excellent Tokio documentation.

## Contributing

As this is a learning project, contributions, suggestions, and feedback are welcome! Feel free to:
- Open issues for bugs or feature requests
- Submit pull requests
- Share your improvements or learning experiences

---

**Note**: This is a learning project and not intended for production use. For production HTTP servers, consider using battle-tested solutions like [Actix-web](https://actix.rs/), [Rocket](https://rocket.rs/), or [Axum](https://github.com/tokio-rs/axum).
