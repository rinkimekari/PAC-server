use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    
    // TODO: cli interface first
    // TODO: multithread

    let listener = TcpListener::bind("127.0.0.1:42069").unwrap();

    loop {
        if let Ok((s, a)) = listener.accept() {
            println!("Connection from: {:?}", a);
            handle_connection(s);
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // works better than string idk

    if let Ok(_) = stream.read(&mut buffer) {
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        let response = "\nPAC SERVER OK\n";
        stream.write(response.as_bytes()).unwrap();
        println!("Sent response: {}", response);
    }
}
