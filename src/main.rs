use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {

    // NOTE: THIS IS VERY BAD CODE I WILL CHANGE THIS TO BE BETTER LATER
    // THIS IS ONLY FOR TESTING PURPOSES RIGHT NOW

    // TODO: send signal for stuff like closing stream
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
    loop {
        let mut buffer = [0; 1024]; // works better than string idk

        if let Ok(_) = stream.read(&mut buffer) {
            let buffer = String::from_utf8_lossy(&buffer[..]);
            println!("Request: {}", buffer);

            let prot_init = "PAC MESSAGE ->";
            let response = if buffer.trim().starts_with(prot_init) {
                "\nPAC SERVER OK\n"
            } else {
                "\nINVALID REQUEST\n"
            };

            // TODO: make better
            if let Err(_) = stream.write(response.as_bytes()) {
                return;
            }
            println!("Sent response: {}", response);
        }
    }
}
