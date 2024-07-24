use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    println!("Building a server!");

    let listner = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listner.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    //prints the HTTP request
    println!("Request: {http_request:#?}");

    //make the response
    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    let contents = fs::read_to_string("hello.html").unwrap();
    let len = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{contents}");

    //send the response
    stream.write_all(response.as_bytes()).unwrap();
}
