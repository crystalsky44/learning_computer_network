use std::net::TcpListener;

use web_server::run;

fn main() {
    println!("Ready for service!");
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    run(listener);
}

