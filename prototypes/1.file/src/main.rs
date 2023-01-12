use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();

    let mut buffer = String::new();

    println!("Enter anything");

    stdin.read_line(&mut buffer).unwrap();

    let mut file = std::fs::File::create("output").unwrap();

    file.write_all(buffer.as_bytes()).unwrap();

    println!("Wrote to file, bye!");
}
