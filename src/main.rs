use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();

    for stream in listener.incoming() {
        let s = stream.unwrap();
        thread::spawn(|| handle_connection(s));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_first_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_first_line[..] {
        "GET / HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 ok", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let request_first_line = buf_reader.lines().next().unwrap().unwrap();
//
//     thread::sleep(Duration::from_secs(5));
//
//     if request_first_line == "GET / HTTP/1.1" {
//         let status_line = "HTTP/1.1 200 ok";
//         let contents = fs::read_to_string("hello.html").unwrap();
//         let length = contents.len();
//
//         let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
//
//         stream.write_all(response.as_bytes()).unwrap();
//     } else {
//         let status_line = "HTTP/1.1 404 NOT FOUND";
//         let contents = fs::read_to_string("404.html").unwrap();
//         let length = contents.len();
//         let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
//
//         stream.write_all(response.as_bytes()).unwrap();
//     }
// }

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let lines: Vec<String> = buf_reader
//         .lines()
//         .map(|x| x.unwrap())
//         .take_while(|line| !line.is_empty())
//
//         .collect();
//     let status_line = "HTTP/1.1 200 ok";
//     let contents = fs::read_to_string("hello.html").unwrap();
//     let length = contents.len();
//
//     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
//
//     stream.write_all(response.as_bytes()).unwrap();
// }
