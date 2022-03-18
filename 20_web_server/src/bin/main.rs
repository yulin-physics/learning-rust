use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use web_server::ThreadPool;
fn main() {
    // Listen to TCP connection
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // A thread pool allows you to process connections concurrently, increasing the throughput of your server
    let pool = ThreadPool::new(4);

    // accept only two requests before gracefully shutting down the server
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // declare a buffer on the stack 1024 bytes in size
    let mut buffer = [0; 1024];
    // read bytes from TcpStream into buffer
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
    // flush will wait and prevent the program from continuing until all the bytes are written to the connection
    stream.flush().unwrap();
}
