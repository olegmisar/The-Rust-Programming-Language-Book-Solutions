use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Read the request
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // Send the response
    let get = b"GET / HTTP/1.1\r\n";
    let (status, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let headers = "";
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}\r\n{}\r\n{}", status, headers, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
