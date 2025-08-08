use std::{error, fmt};

use crate::token::Pos;

#[derive(Debug)]
pub enum Error {
    // Lexing errors
    InvalidNumFormat(Pos),
    UnexpectedChar(Pos, char),
    UnsupportedOperator(Pos, &'static str),
    UnterminatedStr(Pos),   // Indicates the starting position of the string

    // Parsing errorss
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidNumFormat(pos) =>
                write!(f, "Lexing Error: Invalid number format at {pos}"),
            Error::UnexpectedChar(pos, c) =>
                write!(f, "Lexing Error: Unexpected character `{c}` at {pos}"),
            Error::UnsupportedOperator(pos, s) =>
                write!(f, "Lexing Error: Unsupported operator `{s}` at {pos}"),
            Error::UnterminatedStr(pos) =>
                write!(f, "Lexing Error: Unterminated string starting at {pos}"),
        }
    }
}

impl error::Error for Error {}
