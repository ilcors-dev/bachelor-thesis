use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub(crate) struct Server {}

impl Server {
    pub fn new() -> Self {
        Server {}
    }

    pub fn run(&self) {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

        println!("Listening on port 8080...");

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            Server::handle_connection(stream);
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 512];

        stream.read(&mut buffer).unwrap();

        let response = &String::from_utf8_lossy(&buffer[..]);

        stream.write(response.clone().as_bytes()).unwrap();
        stream.flush().unwrap();

        println!("Request: {}", response.clone());
    }
}
