use std::env::args;
use std::fs::read_to_string;
use std::io::{self, BufRead, Write};
use std::process;

fn main() {
    let args: Vec<String> = args().collect();
    println!("args: {:?}, len={}", args, args.len());

    match args.len() {
        1 => run_prompt(),
        2 => {
            run_file(&args[1]);
        }
        _ => {
            println!("Usage: rlox [script]");
            process::exit(64)
        }
    }
}

fn run_file(path: &String) {
    let source = read_to_string(path).unwrap();
    run(source)
}

fn run(source: String) {
    println!("{}", source)
}

fn run_prompt() {
    print!("> ");
    let _ = io::stdout().flush();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let Ok(line) = line else {
            break;
        };
        if line.is_empty() {
            break;
        }
        run(line);
        print!("> ");
        let _ = io::stdout().flush();
    }
}
