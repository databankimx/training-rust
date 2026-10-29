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

fn handle_connection(stream: TcpStream) {
    // BufReader adds buffering, and lines splits the data at each newline.
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        // A blank line marks the end of the request, so the lines after it are not read.
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");

    // Try: open http://127.0.0.1:7878/test in the browser. Which part of the first line changes?
}
