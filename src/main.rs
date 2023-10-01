use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

mod myhttp;
use myhttp::{HTTPMethod, HTTPRequest, NOT_FOUND_RESPONSE, OK_RESPONSE};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => handle_connection(&mut stream),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(stream: &mut TcpStream) {
    let mut request_buffer = [0; 512];
    stream.read(&mut request_buffer).unwrap();
    let request_str = std::str::from_utf8(&request_buffer).unwrap();

    let request = parse_request(request_str);

    let response = match request.path.as_str() {
        "/" => String::from(OK_RESPONSE),
        _ => String::from(NOT_FOUND_RESPONSE),
    };

    let _ = stream.write_all(response.as_bytes());
}

fn parse_request(request_string: &str) -> HTTPRequest {
    let mut lines: Vec<String> = Vec::new();
    request_string
        .lines()
        .for_each(|ln| lines.push(ln.to_string()));

    let mut first_line = lines[0].split(" ");

    let method = match first_line.next().unwrap() {
        "GET" => HTTPMethod::GET,
        "POST" => HTTPMethod::POST,
        _ => HTTPMethod::INVALID,
    };

    println!("Method: {}", method);

    let path = first_line.next().unwrap();
    println!("Path: {}", method);

    return HTTPRequest::new(method, path.to_string());
}
