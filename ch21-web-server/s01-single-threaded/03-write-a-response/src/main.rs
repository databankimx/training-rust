use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    // The request is read, so the connection can be answered. Its contents aren't used yet.
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // A tiny successful response: a status line, a blank line, and no body.
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    // write_all sends the bytes down the connection. The stream is then closed when it's dropped.
    stream.write_all(response.as_bytes()).unwrap();

    // Try: change 200 OK to 404 NOT FOUND in the response string. Does the browser show anything different?
}
