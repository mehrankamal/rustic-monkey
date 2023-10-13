use std::io;
use std::io::{BufRead, Write};
use crate::{lexer, token};

pub fn start<R: BufRead, W: Write>(mut input: R, mut output: W) -> io::Result<()> {
    let prompt = ">> ";
    let mut line = String::new();

    loop {
        write!(output, "{}", prompt)?;
        output.flush()?;

        input.read_line(&mut line)?;

        if line.is_empty() {
            return Ok(());
        }

        let mut l = lexer::Lexer::new(line.as_str());

        let mut token = l.next_token();
        while token != token::Token::Eof {
            write!(output, "{:?}\n", token).unwrap();

            token = l.next_token();
        }

        line.clear();
    }
}