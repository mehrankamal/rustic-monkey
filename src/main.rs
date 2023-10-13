use std::io;

mod token;
mod lexer;
mod repl;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    if let Err(e) = repl::start(stdin.lock(), stdout.lock()) {
        eprintln!("Error: {}", e);
    }
}
