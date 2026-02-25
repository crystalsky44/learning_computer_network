#[cfg(all(tests, feature = "old_tests"))]
mod old_tests {
    use super::*;

    #[test]
    #[ignore]
    fn accept() {
        let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

        let mut stream = accept_connection(listener);

        let mut buffer = [0; 1024];
        let bytes_read = stream.peek(&mut buffer).expect("three letters");
        let peeked = String::from_utf8_lossy(&buffer[..bytes_read]);

        let requested_method: String = peeked.chars().take(3).collect();
        println!("peeked message: {requested_method}");
        assert_eq!(requested_method, "GET".to_string());
    }

    #[test]
    #[ignore]
    fn message() {
        let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
        let mut stream = accept_connection(listener);

        let request_message = get_message(&mut stream);

        let tester: String = request_message[0].chars().take(3).collect();
        assert_eq!("GET".to_string(), tester);
    }

    #[test]
    #[ignore]
    fn requested_file() {
        let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
        let mut stream = accept_connection(listener);
        let raw_message = get_message(&mut stream);
        
        let parsed_message = parse_message(raw_message);

        assert_eq!(parsed_message, PathBuf::from("/hello_world.html"));
    }

    #[test]
    #[ignore]
    fn response_message() {
        let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
        let mut stream = accept_connection(listener);
        let raw_message = get_message(&mut stream);
        let parsed_message = parse_message(raw_message);

        let response_message = generate_response_message(parsed_message);
        println!("{response_message}");
    }

    #[test]
    fn test_send_message() {
        let listener = tcplistener::bind("127.0.0.1:8000").unwrap();
        let mut stream = accept_connection(listener);
        let raw_message = get_message(&mut stream);
        let parsed_message = parse_message(raw_message);
        let response_message = generate_response_message(parsed_message);

        send_message(stream, response_message);
    }
}
