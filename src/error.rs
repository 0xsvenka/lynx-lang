use std::{error, fmt};

use crate::token::{Pos, Span};

#[derive(Debug)]
pub enum Error {
    // Lexing errors
    InvalidNumFormat(Span),
    UnexpectedChar(Pos, char),
    UnsupportedOperator(Span, &'static str),
    UnterminatedStr(Pos),       // Indicates the starting position of the string

    // Parsing errors
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidNumFormat(span) =>
                write!(f, "Lexing Error: Invalid number format at {span}"),
            Error::UnexpectedChar(pos, c) =>
                write!(f, "Lexing Error: Unexpected character `{c}` at {pos}"),
            Error::UnsupportedOperator(span, s) =>
                write!(f, "Lexing Error: Unsupported operator `{s}` at {span}"),
            Error::UnterminatedStr(pos) =>
                write!(f, "Lexing Error: Unterminated string starting at {pos}"),
        }
    }
}

impl error::Error for Error {}
