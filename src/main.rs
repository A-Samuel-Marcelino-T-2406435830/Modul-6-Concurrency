mod lib;

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use lib::ThreadPool;

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

struct HttpResponse {
    status_line: String,
    body: String
}

impl HttpResponse {
    fn response_to_string(&self) -> String {
        let length = self.body.len();
        format!("{}\r\nContent-Length: {}\r\n\r\n{}",
                self.status_line, length, self.body)
    }
}

fn get_response(request_line: &str) -> HttpResponse {
    match request_line {
        "GET / HTTP/1.1" => HttpResponse {
            status_line: "HTTP/1.1 200 OK".to_string(),
            body: fs::read_to_string("hello.html").unwrap(),
        },

        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            HttpResponse {
                status_line: "HTTP/1.1 200 OK".to_string(),
                body: fs::read_to_string("hello.html").unwrap(),
            }
        }

        _ => HttpResponse {
            status_line: "HTTP/1.1 404 NOT FOUND".to_string(),
            body: fs::read_to_string("bad.html").unwrap(),
        },
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let response = get_response(&request_line);

    stream.write_all(response.response_to_string().as_bytes()).unwrap();
}