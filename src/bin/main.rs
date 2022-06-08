use std::{fs, thread};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}


fn handle_connection(mut stream: TcpStream) {
    // buffer big enough to read contents of stream
    // if we needed to read arbitrary-sized streams, buffer management
    // would be more complicated
    let mut buffer = [0; 1024];
    stream.read_exact(&mut buffer).unwrap();

    let get_request_text = b"GET / HTTP/1.1\r\n";
    let sleep_request_text = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get_request_text) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep_request_text) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
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

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}