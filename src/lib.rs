#[warn(clippy::pedantic)]
/// server side
use std::io::{BufRead, BufReader, Read, Write};
use std::fs::File;
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};

use anyhow::Result;

fn run(listener: TcpListener) {
    // accept connection request
    let mut stream = accept_connection(listener);

    // retrieve a client message
    let raw_message = get_message(&mut stream);

    // parse message (it's just getting the path to the file...)
    let parsed_message = parse_message(raw_message);

    // generate response message
    let response_message = generate_response_message(parsed_message);

    send_message(stream, response_message);
}

fn accept_connection(listener: TcpListener) -> TcpStream {
    let (stream, address) = listener.accept().unwrap();
    println!("Connected to {address}");
    stream
}

// retrieves a client sent message
fn get_message(stream: &mut TcpStream) -> Vec<String> {
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
    let mut request_target = raw_message[0]
        .split_whitespace()
        .nth(1)
        .unwrap();

    request_target = &request_target[1..];
    let mut path = PathBuf::from(request_target);
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

fn send_message(mut stream: TcpStream, response_message: String) {
    stream.write_all(response_message.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_test() {
        let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
        run(listener);
    }
}
