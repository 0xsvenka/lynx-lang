use std::{process::abort};

use crate::{lexer::Lexer, parser::Parser};

mod token;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let src = std::fs::read_to_string(path).expect("Failed to read file");

    let mut lexer = Lexer::new(&src);
    let tokens: Vec<token::Token> = Vec::new();
    let mut parser = Parser::new(&tokens);
}
