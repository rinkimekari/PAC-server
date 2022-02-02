use std::io::prelude::*;
use local_ip_address::local_ip;
use std::net::{TcpListener, TcpStream, SocketAddr};

fn main() {

    // NOTE: THIS IS VERY BAD CODE I WILL CHANGE THIS TO BE BETTER LATER
    // THIS IS ONLY FOR TESTING PURPOSES RIGHT NOW

    // TODO: send signal for stuff like closing stream
    // TODO: cli interface
    // TODO: multithread

    let local_ip = local_ip().expect("Couldn't get local IP address. Check your internet connection.");
    let socket = SocketAddr::new(local_ip, 42069);
    let listener = TcpListener::bind(socket).unwrap();

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
