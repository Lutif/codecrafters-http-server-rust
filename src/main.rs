// Uncomment this block to pass the first stage
use std::{
    io::{Read, Write},
    net::TcpListener,
    str::from_utf8,
};
const ALLOWED_URL: &str = "/";
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let mut buf = [0; 1024];
                _stream.read(&mut buf).unwrap();
                let req = from_utf8(&mut buf).unwrap();
                let Some(first) = req.lines().next() else {
                    let response = "HTTP/1.1 200 OK\r\n\r\n";
                    let _ = _stream.write(response.as_bytes());
                    return;
                };
                println!("first {}", first);
                let Some(path) = first.split_whitespace().nth(1) else {
                    let response = "HTTP/1.1 400 Bad Request\r\n\r\n";
                    let _ = _stream.write(response.as_bytes());
                    return;
                };
                println!("path {}", path);
                if path == "/" {
                    let response = "HTTP/1.1 200 OK\r\n\r\n";
                    let _ = _stream.write(response.as_bytes());
                } else {
                    let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                    let _ = _stream.write(response.as_bytes());
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}