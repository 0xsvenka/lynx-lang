use std::collections::HashSet;

use crate::{lexer::Lexer, parser::Parser, sym_table::OpTable};

mod ast;
mod error;
mod lexer;
mod parser;
mod sym_table;
mod token;
mod token_stream;

fn main() {
    // TODO: Handle the situations where wrong args are given
    let path = std::env::args_os().nth(1).unwrap();
    let src = std::fs::read_to_string(path).expect("Failed to read file");

    let op_table = OpTable::new(HashSet::from([
        "=", "=>", "->", "+", "-", "*", "/", "^", "==", "!=", "<", "<=", ">", ">=", "&&", "||",
    ]));
    let lexer = Lexer::new(&src, &op_table);
    let mut _parser = Parser {};
    for token in lexer.tokenize().unwrap().buffer {
        println!("{}", token);
    }
}
