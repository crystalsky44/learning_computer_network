use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("Ready to serve...");
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    if let Ok((stream, address)) = listener.accept() {
        println!("Connection established with {address}!");
        print_message(stream);
    } else {
        println!("couldn't get client...");
    }
}

fn print_message(stream: TcpStream) {
   let mut reader = BufReader::new(stream);

   let mut message = String::new();
   reader.read_line(&mut message).unwrap();

   println!("Received: {}", message.trim());
}

