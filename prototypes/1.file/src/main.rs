use std::env;
use std::fs;
use std::io::{Read, Write};

fn process(input_fname: &str, output_fname: &str) -> Result<(), String> {
    // apertura dei file, stiamo interagendo con il filesystem
    // è necessario che l'host dia le capability necessarie al programma e se non lo fa
    // il metodo map_err() di rust causerà un errore a runtime facendo terminare il programma
    let mut input = fs::File::open(input_fname)
        .map_err(|err| format!("error opening input {}: {}", input_fname, err))?;

    let mut contents = Vec::new();

    input
        .read_to_end(&mut contents)
        .map_err(|err| format!("read error: {}", err))?;

    let mut output = fs::File::create(output_fname)
        .map_err(|err| format!("error opening output {}: {}", output_fname, err))?;

    output
        .write_all(&contents)
        .map_err(|err| format!("write error: {}", err))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() < 3 {
        // interagisce con lo stderr tramite la system call fd_write
        eprintln!("usage: {} <from> <to>", program);
        return;
    }

    if let Err(err) = process(&args[1], &args[2]) {
        eprintln!("{}", err);
        return;
    }

    println!("done!");
}
