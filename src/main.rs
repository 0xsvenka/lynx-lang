use crate::{lexer::Lexer};

mod token;
mod expr;
mod error;
mod lexer;
// mod parser;

fn main() {
    // TODO: Handle the situations where wrong args are given
    let path = std::env::args_os().nth(1).unwrap();
    let src = std::fs::read_to_string(path).expect("Failed to read file");

    let mut lexer = Lexer::new(&src);
    // let mut parser = Parser::new(lexer);
    for result in lexer {
        match result {
            Ok(token) =>
                println!("{token:?}"),
            Err(e) => {
                println!("{e}");
                break;
            }
        }
    }
}
