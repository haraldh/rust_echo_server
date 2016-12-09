use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use std::io::Write;
use std::time::Duration;

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.set_read_timeout(Some(Duration::new(60, 0))).unwrap();
    stream.set_write_timeout(Some(Duration::new(60, 0))).unwrap();

    loop {
        // clear out the buffer so we don't send garbage
        let len = match stream.read(&mut buf) {
            Err(_) => break,
            Ok(m) => {
                if m == 0 {
                    // we've got an EOF
                    break;
                }
                m
            }
        };
        match stream.write_all(&buf[..len]) {
            Err(_) => break,
            Ok(_) => continue,
        }
    }
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
