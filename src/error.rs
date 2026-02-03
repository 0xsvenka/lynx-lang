use std::{error, fmt};

use crate::token::Span;

/// Errors that may occur during the compiling process,
/// including lexing errors, parsing errors, etc.
#[derive(Debug)]
pub enum Error {
    // Lexing errors
    /// Empty character literal.
    EmptyCharLit(Span),
    /// Invalid number literal format.
    InvalidNumLitFormat(Span),
    /// Multiple characters in character literal.
    MultipleCharsInCharLit(Span),
    /// Unexpected character.
    UnexpectedChar(Span),
    /// Unknown escape sequence.
    UnknownEscapeSeq(Span),
    /// Unterminated character/string literal.
    UnterminatedCharOrStrLit(Span),
    // Parsing errors
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::EmptyCharLit(span) => {
                write!(f, "Lexing Error: Empty character literal at {}", span)
            }
            Error::InvalidNumLitFormat(span) => {
                write!(f, "Lexing Error: Invalid number literal format at {}", span)
            }
            Error::MultipleCharsInCharLit(span) => write!(
                f,
                "Lexing Error: Multiple characters in character literal at {}",
                span
            ),
            Error::UnexpectedChar(span) => {
                write!(f, "Lexing Error: Unexpected character at {}", span)
            }
            Error::UnknownEscapeSeq(span) => {
                write!(f, "Lexing Error: Unknown escape sequence at {}", span)
            }
            Error::UnterminatedCharOrStrLit(span) => {
                write!(
                    f,
                    "Lexing Error: Unterminated character/string literal at {}",
                    span
                )
            }
        }
    }
}

impl error::Error for Error {}
