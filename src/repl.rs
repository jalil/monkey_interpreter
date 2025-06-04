use crate::{lexer::Lexer, token::TokenKind};
use std::io::{Stdin, Stdout, Write};

pub fn start(stdin: Stdin, mut stdout: Stdout) {
    loop {
        //Prompt user
        if write!(stdout, ">> ").is_err() || stdout.flush().is_err() {
            eprintln!("Failed to write prompt or flush output.");
            return;
        }

        //Read user input
        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            writeln!(stdout, "Error reading input").ok();
            return;
        }

        //Lex and print tokens
        let mut lexer = Lexer::new(input.as_str());

        loop {
            let token = lexer.next_token();
            if token.kind == TokenKind::Eof {
                break;
            }
            if writeln!(stdout, "{token:?}").is_err() {
                eprintln!("Failed to write token.");
                return;
            }
        }
    }
}
