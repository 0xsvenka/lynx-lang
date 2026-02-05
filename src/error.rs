use std::{error, fmt};

use crate::token::Span;

/// Kind of an error.
#[derive(Debug)]
pub enum ErrorKind {
    // Lexing errors
    EmptyCharLit,
    InvalidNumLitFormat,
    MultipleCharsInCharLit,
    UnexpectedChar,
    UnknownEscapeSeq,
    UnterminatedCharOrStrLit,
    // Parsing errors
}

/// Error occurring during the compilation process.
#[derive(Debug)]
pub struct Error(
    /// Kind of the error.
    pub ErrorKind,
    /// Position in Lynx source where the error occurred.
    pub Span,
);

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::EmptyCharLit => write!(f, "empty character literal"),
            ErrorKind::InvalidNumLitFormat => write!(f, "invalid number literal format"),
            ErrorKind::MultipleCharsInCharLit => {
                write!(f, "multiple characters in character literal")
            }
            ErrorKind::UnexpectedChar => write!(f, "unexpected character"),
            ErrorKind::UnknownEscapeSeq => write!(f, "unknown escape sequence"),
            ErrorKind::UnterminatedCharOrStrLit => {
                write!(f, "unterminated character/string literal")
            }
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {} at {}", self.0, self.1)
    }
}

impl error::Error for Error {}
