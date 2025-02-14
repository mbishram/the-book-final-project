use hello::ThreadPool;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let content_type = " text/html; charset=utf-8";

    let response =
            format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type:{content_type}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
