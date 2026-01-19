#[warn(clippy::pedantic)]
/// server side

use std::io::{Read, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("Ready to serve...");
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    if let Ok((stream, address)) = listener.accept() {
        println!("Connection established with {address}!");
        let path = return_message(stream);
        println!("{path}");
    } else {
        println!("couldn't get client...");
    }
}

fn return_message(stream: TcpStream) -> String {
   let mut reader = BufReader::new(stream);

   let mut message:Vec<String> = Vec::new();
   // reader.read_to_string(&mut message).unwrap();

   for line in reader.lines().take(3) {
       message.push(line.expect("some error"));
   }

   println!("{message:?}");

   message[0].split_whitespace().nth(1).unwrap().to_string()
}

