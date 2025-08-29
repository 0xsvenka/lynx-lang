use std::{error, fmt};

use crate::token::{Pos, Span};

#[derive(Debug)]
pub enum Error {
    // Lexing errors
    InvalidNumFormat(Span),
    UnexpectedChar(Pos, char),
    UnknownEscapeSeq(Span, String),
    /// The `Pos` indicates the starting position of the string
    UnterminatedStr(Pos),

    // Parsing errors
    UnexpectedToken(String),
    UnexpectedEOF,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidNumFormat(span) =>
                write!(f, "Lexing Error: Invalid number format at {span}"),
            Error::UnexpectedChar(pos, c) =>
                write!(f, "Lexing Error: Unexpected character `{c}` at {pos}"),
            Error::UnknownEscapeSeq(span, s) =>
                write!(f, "Lexing Error: Unknown escape sequence `{s}` at {span}"),
            Error::UnterminatedStr(pos) =>
                write!(f, "Lexing Error: Unterminated string starting at {pos}"),
            _ => write!(f, ""),     // TODO: Add support for other errors
        }
    }
}

impl error::Error for Error {}
