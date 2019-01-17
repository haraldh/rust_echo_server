use std::io;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn handle_client(mut stream: TcpStream) {
    io::copy(&mut stream.try_clone().unwrap(), &mut stream);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:12345").unwrap();
    for stream in listener.incoming() {
        match stream {
            Err(e) => println!("failed: {}", e),
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
        }
    }
}
