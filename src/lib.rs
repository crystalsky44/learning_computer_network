#[warn(clippy::pedantic)]
/// server side

use std::io::{Read, Write, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

use anyhow::Result;

fn run(listener: TcpListener) {
    // accept connection request
    let stream = accept_connection(listener);
}

fn accept_connection(listener: TcpListener) -> TcpStream {
    let (stream, address) = listener.accept().unwrap();
    println!("Connected to {address}");
    stream
}

// takes byte-stream and returns it in String
fn get_message(stream: TcpStream) -> String {
    // buff the byte-stream data
    let mut reader = BufReader::new(stream);

    // convert the buffed bytes into String
    reader.to_string()


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
}
