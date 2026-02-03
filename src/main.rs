use crate::lexer::tokenize;

mod ast;
mod error;
mod lexer;
mod parser;
mod token;

fn main() {
    // TODO: Handle the situations where wrong args are given
    let path = std::env::args_os().nth(1).unwrap();
    let src = std::fs::read_to_string(path).expect("Failed to read file");

    for token in tokenize(&src).unwrap() {
        println!("{}", token);
    }
}
