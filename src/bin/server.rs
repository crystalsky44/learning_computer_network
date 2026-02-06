#[warn(clippy::pedantic)]
/// server side

use std::io::{Read, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

use anyhow::Result;

fn main() {
    println!("Ready to serve...");
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    if let Ok(result) = run(listener) {
        println!("Success!!");
    } else {
        println!("failed");
    }

    /*
    if let Ok((stream, address)) = listener.accept() {
        println!("Connection established with {address}!");
        let parsed_message = parse_message(stream);
        let response_message = generate_response(parse_message);
        println!("{response_message}");
        send_response(response_message).unwrap();
    } else {
        println!("couldn't get client...");
    }
    */
}

fn run(listener: TcpListener) -> Result<String> {
    // accept connection request
    let stream = accept_connection(listener);
    //
    // read the byte stream
    let raw_message = get_message(stream);
    // 
    // parse the byte stream
    let parsed_message = parse_message(raw_message);
    //
    // generate response message
    let reponse_message = generate_response_message(parsed_message);
    //
    // return response message to main
    Ok(send_message(response_message)?)
}

// take listener and return message
fn accept_connection(listener: TcpListener) -> TcpStream {
    let stream: TcpStream;

    if let Ok((stream, address)) = listener.accept() {
        println!("Connection established with {address}!");
    } else {
        println!("couldn't get client...");
    }

    stream
}

fn get_message(stream: TcpStream) -> String {
   let mut reader = BufReader::new(stream);
   let mut message = String::new();

   // convert byte-stream to String
}

// take the raw byte stream and parse the message
fn parse_message(raw_message: String) -> String {
    // get the thrid line out of the raw_message
    let parsing_line = raw_message.lines() 
    println!("{parsing_line:?}");

    // extract and return the 2nd element in the parsing_line
    message[0].split_whitespace().nth(1).unwrap().to_string()
}

// generates an http response message
fn generate_response(path: impl AsRef<Path>) -> String {
    // open file with the passed String or the above <- study
    let mut content = String::new();
    let file = File::open(path).unwrap(); // check this one before

    // create response specific message
    let start_line = String::from("HTTP/1.1 200 OK");
    let field_line = Stirng::from("Connection: close");
    let content = file.read_to_string();

    format!("{}\r\n{}\r\n\r\n{}", start_line, field_line, content)
}

// sends the response message through the conncetion
fn send_response(response: String, stream: TcpStream) -> Result<()> {
    stream.write_all(response.as_bytes());
    Ok(())
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
        assert_eq!(requested_method, "GET".to_string());
    }
}
