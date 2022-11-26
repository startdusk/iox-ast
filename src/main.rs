use std::fs::read_to_string;
use std::io::{stdin, stdout, Write};
use std::{env::args, process::exit};

fn main() {
    let args: Vec<String> = args().collect();
    println!("args: {:?}, len={}", args, args.len());

    match args.len() {
        2 => run_file(&args[1]),
        len if len > 2 => {
            println!("Usage: rlox [script]");
            exit(64)
        }
        _ => run_prompt(),
    }
}

fn run_file(path: &String) {
    let source = read_to_string(path).unwrap();
    run(source)
}

fn run(_source: String) {}

fn run_prompt() {
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                match input {
                    "exit" => {
                        print!("Bye~");
                        return;
                    }
                    _ => {
                        println!("{input}");
                    }
                }
            }
            Err(error) => {
                print!("error: {error}");
                continue;
            }
        }
    }
}
