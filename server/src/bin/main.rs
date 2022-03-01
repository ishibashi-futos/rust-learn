extern crate logger;
extern crate server;

use server::ThreadPool;

use std::{
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use logger::logger::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(3) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    logger::logger::info("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let request = String::from_utf8_lossy(&buffer[..]);
    info(&format!("Request: {}", request));
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "www/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "www/errors/404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    let _ = stream.flush();
}
