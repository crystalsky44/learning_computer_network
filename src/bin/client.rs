use std::io;
use std::io::Write;
use std::net::TcpStream;

fn main() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8000") {
        println!("Connected to the server!");

        let message = get_user_input("Enter your message");
        stream.write_all(message.as_bytes()).unwrap();

    } else {
        println!("Couldn't connect to the server...");
    }
}

fn get_user_input(prompt_message: &str) -> String {
    print!("{prompt_message}: ");

    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    input.trim_end().to_string()
}
