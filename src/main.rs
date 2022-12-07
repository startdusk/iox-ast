use std::env::args;
use std::fs::read_to_string;
use std::io::{self, BufRead, Write};
use std::process;

fn main() {
    let args: Vec<String> = args().collect();
    println!("args: {:?}, len={}", args, args.len());
    let mut lox = Lox::new();
    lox.play(args)
}

struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }
    pub fn play(&mut self, args: Vec<String>) {
        match args.len() {
            1 => self.run_prompt(),
            2 => {
                self.run_file(&args[1]);
            }
            _ => {
                println!("Usage: rlox [script]");
                process::exit(64)
            }
        }
    }
}

impl Lox {
    fn run_file(&self, path: impl Into<String>) {
        let source = read_to_string(path.into()).unwrap();
        self.run(source);
        if self.had_error {
            process::exit(65)
        }
    }

    fn run(&self, source: String) {
        println!("{}", source)
    }

    fn run_prompt(&mut self) {
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
            self.run(line);
            self.had_error = false;
            print!("> ");
            let _ = io::stdout().flush();
        }
    }

    fn error(&mut self, line: usize, message: impl Into<String>) {
        self.report(line, "", message)
    }

    fn report(&mut self, line: usize, position: impl Into<String>, message: impl Into<String>) {
        println!(
            "[line {}] Error: {}\n\t{}",
            line,
            message.into(),
            position.into(),
        );
        self.had_error = true;
    }
}
