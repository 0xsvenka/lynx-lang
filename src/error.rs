use std::{error, fmt};

use crate::token::{Pos, Span};

/// All errors that may occur during the whole process,
/// including lexing errors, parsing errors, ...
#[derive(Debug)]
pub enum Error {
    // Lexing errors
    /// Empty character literal.
    EmptyCharLit(Span),
    /// Invalid format of a number literal.
    InvalidNumLitFormat(Span),
    /// Multiple characters inside a single character literal.
    MultipleCharsInCharLit(Span),
    /// Character appearing illegally, which cannot be lexed.
    UnexpectedChar(Pos),
    /// Unknown escape sequence in a character or string literal.
    UnknownEscapeSeq(Span),
    /// Character literal without closing quote.
    /// The [`Pos`] indicates the starting position of the literal.
    UnterminatedCharLit(Pos),
    /// String literal without closing quote.
    /// The [`Pos`] indicates the starting position of the literal.
    UnterminatedStrLit(Pos),

    // Parsing errors
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::EmptyCharLit(span) =>
                write!(f, "Lexing Error: Empty character literal at {span}"),
            Error::InvalidNumLitFormat(span) =>
                write!(f, "Lexing Error: Invalid number literal format at {span}"),
            Error::MultipleCharsInCharLit(span) => 
                write!(f, "Lexing Error: Multiple characters in character literal at {span}"),
            Error::UnexpectedChar(pos) =>
                write!(f, "Lexing Error: Unexpected character at {pos}"),
            Error::UnknownEscapeSeq(span) =>
                write!(f, "Lexing Error: Unknown escape sequence at {span}"),
            Error::UnterminatedCharLit(pos) =>
                write!(f, "Lexing Error: Unterminated character literal starting at {pos}"),
            Error::UnterminatedStrLit(pos) =>
                write!(f, "Lexing Error: Unterminated string literal starting at {pos}"),
        }
    }
}

impl error::Error for Error {}
