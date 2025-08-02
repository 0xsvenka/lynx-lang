use std::num::ParseIntError;

use logos::Logos;

#[derive(Default, Debug, Clone, PartialEq)]
enum LexingError {
    InvalidInteger(String),
    NonAsciiCharacter(char),
    #[default]
    Other,
}

/// Error type returned by calling `lex.slice().parse()` to u8.
impl From<ParseIntError> for LexingError {
    fn from(err: ParseIntError) -> Self {
        use std::num::IntErrorKind::*;
        match err.kind() {
            PosOverflow | NegOverflow => LexingError::InvalidInteger("overflow error".to_owned()),
            _ => LexingError::InvalidInteger("other error".to_owned()),
        }
    }
}

impl LexingError {
    fn from_lexer<'src>(lex: &mut logos::Lexer<'src, Token>) -> Self {
        LexingError::NonAsciiCharacter(lex.slice().chars().next().unwrap())
    }
}

#[derive(Logos, Debug, PartialEq)]
// TODO: #[logos(error(LexingError, LexingError::from_lexer))]
#[logos(skip r"[ \t\f]+")]
pub enum Token {
    #[regex("[;\n]")]
    ExprEnd,

    #[token("if")]
    If,

    #[token("then")]
    Then,

    #[token("else")]
    Else,

    #[token("while")]
    While,

    #[token("do")]
    Do,

    #[token("for")]
    For,

    #[token("in")]
    In,

    #[token("(")]
    LP,

    #[token(")")]
    RP,

    #[token(".")]
    Dot,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Id,

    // TODO: #[regex("[0-9]+", |lex| lex.slice().parse())]
    // Integer(u8),
}
