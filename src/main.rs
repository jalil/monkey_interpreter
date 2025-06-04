pub mod ast;
pub mod lexer;
pub mod repl;
pub mod token;
pub mod parser;

use std::io;

use crate::repl::start;
fn main() {
    println!("Monkey see money Do! ");

    start(io::stdin(), io::stdout());
}
