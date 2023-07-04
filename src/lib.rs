use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn plus(one: i32, two: i32) -> i32 {
    one + two
}

pub fn minus(one: i32, two: i32) -> i32 {
    if one > two {
        one - two
    } else {
        -1
    }
}

pub fn run_server(addr:&str, port:&str) {
    let listener = TcpListener::bind(format!("{addr}:{port}")).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(format!("www/{filename}")).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn plus_work() {
        let result = plus(23, 43);
        assert_eq!(result, 23 + 43);
    }
}
