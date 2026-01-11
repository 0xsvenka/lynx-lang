use crate::{lexer::Lexer, parser::Parser};

mod ast;
mod error;
mod lexer;
mod parser;
mod token;
mod token_stream;

fn main() {
    // TODO: Handle the situations where wrong args are given
    let path = std::env::args_os().nth(1).unwrap();
    let src = std::fs::read_to_string(path).expect("Failed to read file");

    let lexer = Lexer::new(&src);
    let mut _parser = Parser {};
    for token in lexer.tokenize().unwrap().buffer {
        println!("{}", token);
    }
}
