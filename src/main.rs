// Uncomment this block to pass the first stage
use std::{
    collections::HashMap, env::{args as get_args, Args}, fs, io::{Read, Write}, net::TcpListener, path::Path, str::from_utf8, thread::spawn as thread_spawn,
    fs::File,
};
const NEW_LINE:&str = "\r\n";
fn parse_headers(req: &str) -> HashMap<&str, &str> {
    req.lines()
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.splitn(2, ':');
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();
            (key, value.trim())
        })
        .collect()
}
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    // Uncomment this block to pass the first stage
    //

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                thread_spawn(move || {
                    let mut buf = [0; 1024];
                _stream.read(&mut buf).unwrap();
                let req = from_utf8(&mut buf).unwrap();
                let mut parts = req.split("\r\n\r\n");
                let _headers = parts.next().unwrap();
                let body = parts.next().unwrap_or("");

                let Some(first) = req.lines().next() else {
                    let response = "HTTP/1.1 200 OK\r\n\r\n";
                    let _ = _stream.write(response.as_bytes());
                    return;
                };
                println!("first {}", first);
                let Some(method) = first.split_whitespace().next() else {
                    let response = "HTTP/1.1 400 Bad Request\r\n\r\n";
                    let _ = _stream.write(response.as_bytes());
                    return;
                };
                let Some(path) = first.split_whitespace().nth(1) else {
                    let response = "HTTP/1.1 400 Bad Request\r\n\r\n";
                    let _ = _stream.write(response.as_bytes());
                    return;
                };
                println!("path {}", path);
                //if path includes echo, return 200 OK
                
                match path {
                    "/" => {
                        let response = format!("HTTP/1.1 200 OK{}{}",NEW_LINE,NEW_LINE);
                        let _ = _stream.write(response.as_bytes());
                    }
                    "/user-agent" => {
                        let headers = parse_headers(req);
                        let user_agent = headers.get("User-Agent").unwrap();
                        let response =format!("HTTP/1.1 200 OK{}Content-Type: text/plain{}Content-Length: {}{}{}{}{}",NEW_LINE,NEW_LINE,user_agent.chars().count(),NEW_LINE,NEW_LINE,user_agent,NEW_LINE);
                        let _ = _stream.write(response.as_bytes());
                    }
                    _ if path.starts_with("/echo") => {
                        let message = path.split("/echo/").nth(1).unwrap();
                        let response =format!("HTTP/1.1 200 OK{}Content-Type: text/plain{}Content-Length: {}{}{}{}{}",NEW_LINE,NEW_LINE,message.chars().count(),NEW_LINE,NEW_LINE,message,NEW_LINE);
                        let _ = _stream.write(response.as_bytes());
                    }
                    _ if path.starts_with("/files") => {
                        let mut args:Args = get_args();
                        let directory = Some(args.nth(2)).unwrap().unwrap();
                        let file_name = path.split("/files/").nth(1).unwrap();
                        let path_name = Path::new(directory.as_str()).join(file_name);

                        match method {
                            "GET" => {
                                print!("Got get request");
                                if !path_name.exists() {
                                    let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                                    let _ = _stream.write(response.as_bytes());
                                    return;
                                }else {
                                    let file_contents = std::fs::read_to_string(path_name).unwrap();
                                    let response =format!("HTTP/1.1 200 OK{}Content-Type: application/octet-stream{}Content-Length: {}{}{}{}{}",NEW_LINE,NEW_LINE,file_contents.chars().count(),NEW_LINE,NEW_LINE,file_contents,NEW_LINE);
                                    let _ = _stream.write(response.as_bytes());
                                }
                            },
                            "POST" => {
                                print!("Got post request");
                                print!("path name {}",body);
                                let mut file = File::create(path_name).unwrap();
                                file.write_all( body.as_bytes()).unwrap();
                                let response = format!("HTTP/1.1 201 OK{}{}",NEW_LINE,NEW_LINE);
                                let _ = _stream.write(response.as_bytes());
                                return ;
                            },
                            _ => {
                                let response = "HTTP/1.1 400 Bad Request\r\n\r\n";
                                let _ = _stream.write(response.as_bytes());
                                return;
                            }
                            
                        }

                        //create path to file using directory and file_name
                        //check if file exists

                        print!("file name {}",file_name);
                        // let response =format!("HTTP/1.1 200 OK{}Content-Type: text/plain{}Content-Length: {}{}{}{}{}",NEW_LINE,NEW_LINE,message.chars().count(),NEW_LINE,NEW_LINE,message,NEW_LINE);
                        // let _ = _stream.write(response.as_bytes());
                    }
                    _ => {
                        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                        let _ = _stream.write(response.as_bytes());
                    }
                    
                }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}