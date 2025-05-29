pub mod lexer;
pub mod token;
pub mod repl;


 use std::io;

use crate::repl::start;
fn main() {
    println!("Monkey see money Do! ");

    start(io::stdin(), io::stdout());
}
