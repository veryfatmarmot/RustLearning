use anyhow::{Context, Result};
use request_handler;
use std::{
    io,
    net::TcpListener,
    sync::{
        Arc,
        atomic::{self, AtomicBool},
    },
};

pub fn run_server(addr: &str, should_be_running: Arc<AtomicBool>) -> Result<()> {
    let listener =
        TcpListener::bind(addr).with_context(|| format!("Failed to bind to http://{addr}"))?;

    listener
        .set_nonblocking(true)
        .context("can't set a listener non-blocking")?;

    println!("Server listening on http://{addr}");

    while should_be_running.load(atomic::Ordering::Acquire) {
        match listener.accept() {
            Ok((stream, _)) => {
                // Set stream back to blocking mode for request handling
                stream
                    .set_nonblocking(false)
                    .context("Failed to set stream to blocking")?;

                if let Err(e) = request_handler::handle_connection(stream) {
                    eprintln!("Connection handling error: {e}");
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                while !wait_for_listener_readable(&listener)? {
                    if !should_be_running.load(atomic::Ordering::Acquire) {
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {e}");
            }
        }
    }

    eprintln!("Server stopped");

    Ok(())
}

#[cfg(unix)]
fn wait_for_listener_readable(listener: &TcpListener) -> io::Result<(bool)> {
    use std::os::unix::io::AsRawFd;

    let fd = listener.as_raw_fd();

    // Block until the socket is readable (POLLIN) => accept() should not WouldBlock.
    let mut pfd = libc::pollfd {
        fd,
        events: libc::POLLIN,
        revents: 0,
    };

    let rc = unsafe { libc::poll(&mut pfd, 1, 10) }; // -1 = wait forever
    if rc == 0 {
        return Ok(false);
    } else if rc == -1 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(windows)]
fn wait_for_listener_readable(listener: &TcpListener) -> io::Result<bool> {
    use std::os::windows::io::AsRawSocket;
    use windows_sys::Win32::Networking::WinSock::{POLLRDNORM, WSAPOLLFD, WSAPoll};

    let sock = listener.as_raw_socket() as usize;

    let mut pfd = WSAPOLLFD {
        fd: sock,
        events: POLLRDNORM as i16, // readable
        revents: 0,
    };

    let rc = unsafe { WSAPoll(&mut pfd as *mut WSAPOLLFD, 1, 10) }; // -1 = wait forever
    if rc == 0 {
        return Ok(false);
    } else if rc == -1 {
        return Err(io::Error::last_os_error());
    }
    Ok(true)
}
