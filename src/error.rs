use std::{error, fmt};

use crate::token::Pos;

/// Errors that may occur during the compiling process,
/// including lexing errors, parsing errors, etc.
#[derive(Debug)]
pub enum Error {
    // Lexing errors
    /// Empty character literal.
    EmptyCharLit(Pos, Pos),
    /// Invalid number literal format.
    InvalidNumLitFormat(Pos, Pos),
    /// Multiple characters in character literal.
    MultipleCharsInCharLit(Pos, Pos),
    /// Unexpected character.
    UnexpectedChar(Pos),
    /// Unknown escape sequence, in character or string literal.
    UnknownEscapeSeq(Pos, Pos),
    /// Unterminated character literal,
    /// character literal without a closing quote in the same line,
    /// starting position indicated by the [`Pos`].
    UnterminatedCharLit(Pos),
    /// Unterminated string literal,
    /// string literal without a closing quote in the same line,
    /// starting position indicated by the [`Pos`].
    UnterminatedStrLit(Pos),
    // Parsing errors
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::EmptyCharLit(start_pos, end_pos) => {
                write!(
                    f,
                    "Lexing Error: Empty character literal at {}-{}",
                    start_pos, end_pos
                )
            }
            Error::InvalidNumLitFormat(start_pos, end_pos) => {
                write!(
                    f,
                    "Lexing Error: Invalid number literal format at {}-{}",
                    start_pos, end_pos
                )
            }
            Error::MultipleCharsInCharLit(start_pos, end_pos) => write!(
                f,
                "Lexing Error: Multiple characters in character literal at {}-{}",
                start_pos, end_pos
            ),
            Error::UnexpectedChar(pos) => write!(f, "Lexing Error: Unexpected character at {pos}"),
            Error::UnknownEscapeSeq(start_pos, end_pos) => {
                write!(
                    f,
                    "Lexing Error: Unknown escape sequence at {}-{}",
                    start_pos, end_pos
                )
            }
            Error::UnterminatedCharLit(pos) => write!(
                f,
                "Lexing Error: Unterminated character literal starting at {pos}"
            ),
            Error::UnterminatedStrLit(pos) => write!(
                f,
                "Lexing Error: Unterminated string literal starting at {pos}"
            ),
        }
    }
}

impl error::Error for Error {}
