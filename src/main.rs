// Uncomment this block to pass the first stage
use std::{ net::TcpListener, io::{Write, Read} };
const ALLOWED_URL: &str = "/";
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = String::new();
                match stream.read_to_string(&mut buffer) {
                    Ok(_) => {
                        let first_line: &str = buffer.lines().next().unwrap();
                        let str_vec:Vec<&str> = first_line.split(" ").collect();
                        let url = str_vec[1];
                        println!("url line: {:?}", url);
                        match url {
                            ALLOWED_URL => {
                                println!("Allowed URL");
                                let response = "HTTP/1.1 200 OK\r\n\r\n";
                                match stream.write(response.as_bytes()) {
                                    Ok(_) => println!("Response sent"),
                                    Err(e) => println!("Failed sending response: {}", e),
                                };
                            },
                            _ => {
                                println!("Not Allowed URL");
                                let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
                                match stream.write(response.as_bytes()) {
                                    Ok(_) => println!("Response sent"),
                                    Err(e) => println!("Failed sending response: {}", e),
                                };
                            }
                            
                        }
                    },

                    Err(e) => println!("Failed reading request: {}", e),
                };
                stream.flush().unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

}
