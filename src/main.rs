use crate::{lexer::Lexer, parser::Parser};

mod error;
mod expr;
mod lexer;
mod parser;
mod token;

fn main() {
    // TODO: Handle the situations where wrong args are given
    let path = std::env::args_os().nth(1).unwrap();
    let src = std::fs::read_to_string(path).expect("Failed to read file");

    let lexer = Lexer::new(&src);
    let mut _parser = Parser {};
    for result in lexer {
        println!("{:?}", result);
    }
}
