#[warn(clippy::pedantic)]
/// server side
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

use anyhow::Result;

fn run(listener: TcpListener) {
    // accept connection request
    let stream = accept_connection(listener);

    // retrieve a client message
    let message = get_message(stream);

    todo!("Still working on the code from this point!");
}

fn accept_connection(listener: TcpListener) -> TcpStream {
    let (stream, address) = listener.accept().unwrap();
    println!("Connected to {address}");
    stream
}

// retrieves a client sent message
fn get_message(stream: TcpStream) -> Vec<String> {
    // buff the client message
    let mut reader = BufReader::new(stream);

    // convert the buffed byte data into Vec<String>
    let request_message: Vec<String> = reader
        .lines() 
        .map(|line| line.unwrap())
        .collect();

    println!("{request_message:?}");
    request_message
}

fn parse_message(raw_message: Vec<String>) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accept() {
        let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

        let stream = accept_connection(listener);

        let mut buffer = [0; 1024];
        let bytes_read = stream.peek(&mut buffer).expect("three letters");
        let peeked = String::from_utf8_lossy(&buffer[..bytes_read]);

        let requested_method: String = peeked.chars().take(3).collect();
        println!("peeked message: {requested_method}");
        assert_eq!(requested_method, "GET".to_string());
    }
    #[test]
    fn message() {
        let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
        let stream = accept_connection(listener);

        let request_message = get_message(stream);

        let tester: String = request_message[0].chars().take(3).collect();
        assert_eq!("GET".to_string(), tester);
    }
}
