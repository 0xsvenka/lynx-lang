use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct Pos(pub usize, pub usize);   // (line, column)

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos() {
        let pos = Pos(0, 1);
        assert_eq!(format!("{pos}"), "0:1");
    }
}
