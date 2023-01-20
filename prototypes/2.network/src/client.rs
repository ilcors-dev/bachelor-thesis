use std::io::{Read, Write};
use std::net::TcpStream;

pub(crate) struct Client {}

impl Client {
    pub fn new() -> Self {
        Client {}
    }

    pub fn run(&self) {
        loop {
            let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
            Client::request(&mut stream);
        }
    }

    fn request(stream: &mut TcpStream) {
        println!("Enter a message to send to the server: ");

        let mut input = String::new();

        let stdin = std::io::stdin();

        stdin.read_line(&mut input).unwrap();

        stream.write(input.as_bytes()).unwrap();
        stream.flush().unwrap();

        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        println!("Response: {}", String::from_utf8_lossy(&buffer[..]));

        stream.flush().unwrap();
    }
}
