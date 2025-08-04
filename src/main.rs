use crate::{lexer::Lexer, token::Token};

mod token;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let src = std::fs::read_to_string(path).expect("Failed to read file");

    let mut lexer = Lexer::new(&src);
    loop {
        match lexer.next_token() {
            Ok(token) => {
                println!("{:?}", token);
                if token == Token::EOF {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Lexer error: {:?}", e);
                break;
            }
        }
    }
}
