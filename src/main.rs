use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}


fn handle_connection(mut stream: TcpStream) {
    // buffer big enough to read contents of stream
    // if we needed to read arbitrary-sized streams, buffer management
    // would be more complicated
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get_request_text = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get_request_text) {
        let contents = fs::read_to_string("hello.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        // handle other request
    }
}