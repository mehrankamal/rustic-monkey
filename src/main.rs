use std::{env, io};
use std::io::Write;

mod token;
mod lexer;
mod repl;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let user = env::var("USER").unwrap();

    write!(stdout.lock(), "Hello {}! This is the Monkey programming language!\n", user).unwrap();
    write!(stdout.lock(), "Feel free to type in commands\n").unwrap();

    if let Err(e) = repl::start(stdin.lock(), stdout.lock()) {
        eprintln!("Error: {}", e);
    }
}
