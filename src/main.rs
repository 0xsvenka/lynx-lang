use std::{process::abort};

use crate::{lexer::Lexer, token::Token};

mod token;
// mod expr;
mod error;
mod lexer;
// mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let src = std::fs::read_to_string(path).expect("Failed to read file");

    let mut lexer = Lexer::new(&src);
    // let mut parser = Parser::new(lexer);
    for result in lexer {
        println!("{result:?}");
    }
}
