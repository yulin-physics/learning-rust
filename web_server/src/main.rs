use std::env;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
fn main() {
    // Listen to TCP connection
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let args: Vec<String> = env::args().collect();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        if args.len() == 1 {
            handle_connection(stream);
        } else if args.len() == 2 {
            handle_connection_with_validation(stream);
        }
    }
}

fn handle_connection_with_validation(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "home.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "home.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection(mut stream: TcpStream) {
    // Reading request
    // 1024 bytes buffer to read in data
    let mut buffer = [0; 1025];
    // Read bytes from the TcpStream and put them in the buffer.
    stream.read(&mut buffer).unwrap();
    // Convert bytes to string and print
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]))

    // Writing response
    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    let contents = fs::read_to_string("home.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
