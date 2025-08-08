use crate::lexer::Lexer;

mod token;
mod expr;
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
        match result {
            Ok(token) => println!("{token:?}"),
            Err(e) => println!("{e}")
        }
    }
}
