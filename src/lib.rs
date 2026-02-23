#[warn(clippy::pedantic)]
/// server side
use std::io::{BufRead, BufReader, Read, Write};
use std::fs::File;
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};

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
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{request_message:#?}");
    request_message
}

// gets the name of file requested of.
// "parse" should be more complicated. But for this project
// I'm just getting the string to call the requested file
fn parse_message(raw_message: Vec<String>) -> PathBuf {
    let current_directory = std::env::current_dir().unwrap();
    let mut path = PathBuf::from(current_directory);
    println!("line after from: {}", path.display());
    let request_target = raw_message[0]
        .split_whitespace()
        .nth(1)
        .unwrap();

    path.push(request_target);
    println!("Current working directory: {}",
        std::env::current_dir().unwrap().display());
    println!("after push: {}", path.display());

    path
}

fn generate_response_message(parsed_message: PathBuf) -> String {
    // open file with the passed String or the above <- study
    let mut file = File::open(parsed_message).unwrap(); // check this one before

    // create response specific message
    let start_line = String::from("HTTP/1.1 200 OK");
    let field_line = String::from("Connection: close");
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    println!("{content}");
    format!("{start_line}\r\n{field_line}\r\n\r\n{content}")
}

fn send_message() {
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

    #[test]
    fn requested_file() {
        let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
        let stream = accept_connection(listener);
        let raw_message = get_message(stream);
        
        let parsed_message = parse_message(raw_message);

        assert_eq!(parsed_message, PathBuf::from("/hello_world.html"));
    }

    #[test]
    fn response_message() {
        let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
        let stream = accept_connection(listener);
        let raw_message = get_message(stream);
        let parsed_message = parse_message(raw_message);

        let response_message = generate_response_message(parsed_message);
        println!("{response_message}");
    }

}
