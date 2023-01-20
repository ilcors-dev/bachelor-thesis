mod client;
mod server;
use std::io;

fn main() {
    println!("Type s to be a server, c to be a client..\n");

    let mut input = String::new();

    let stdin = io::stdin();

    stdin.read_line(&mut input).unwrap();

    if input.trim() == "s" {
        let server = server::Server::new();

        server.run();
    } else if input.trim() == "c" {
        let client = client::Client::new();

        client.run();
    } else {
        println!("Invalid input");
    }
}
