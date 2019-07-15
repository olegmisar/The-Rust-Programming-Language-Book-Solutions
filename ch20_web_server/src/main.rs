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
    let mut buffer = [0; 512];
    let n_bytes_read = stream.read(&mut buffer).unwrap();
    if n_bytes_read != 0 {
        let data = String::from_utf8_lossy(&buffer);
        println!("========= NEW REQUEST =========");
        println!("{}", data);
        println!();
    }
}
