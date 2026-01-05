use std::error::Error;
use std::fs;
use std::io::{BufReader, Error as IoError, prelude::*};
use std::net::{TcpListener, TcpStream};

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:7879")?;

    for stream in listener.incoming() {
        let stream = stream?;

        handle_connection(stream)?;
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");

    if http_request.len() < 1 {
        return Err(Box::new(IoError::new(
            std::io::ErrorKind::InvalidData,
            "Invalid HTTP request",
        )));
    }

    let mut http_request_parts_it = http_request[0].split_whitespace();
    let http_method = http_request_parts_it
        .next()
        .expect("Invalid first header line. Need METHOD");
    let uri = http_request_parts_it
        .next()
        .expect("Invalid first header line. Need URI");

    if http_method != "GET" {
        return Err(Box::new(IoError::new(
            std::io::ErrorKind::InvalidData,
            "Only GET method is supported",
        )));
    }

    let response: Vec<u8> = match uri {
        "/" => get_page_response(),
        "/favicon.ico" => get_favicon_response(),
        _ => get_404_response(),
    }?;

    if let Ok(str) = std::str::from_utf8(&response) {
        println!("Response: {str}");
    } else {
        println!("Response: [binary data]");
    }

    stream.write_all(&response)?;

    Ok(())
}

fn get_404_response() -> Result<Vec<u8>, Box<dyn Error>> {
    Ok("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes().to_vec())
}

fn get_page_response() -> Result<Vec<u8>, Box<dyn Error>> {
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("resources/hello.html")?;
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    Ok(response.into_bytes())
}

fn get_favicon_response() -> Result<Vec<u8>, Box<dyn Error>> {
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read("resources/favicon.ico")?;
    let length = contents.len();

    let header = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");

    let mut response = header.into_bytes();
    response.extend(contents);

    Ok(response)
}
