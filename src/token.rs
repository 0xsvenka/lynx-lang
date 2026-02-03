use std::fmt;

/// Position of a character in Lynx source.
#[derive(Debug, Clone, Copy)]
pub struct Pos(
    /// Line number, starting from `1`.
    pub usize,
    /// Column number, starting from `1`.
    pub usize,
);

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

/// Position of a span of text in Lynx source.
#[derive(Debug)]
pub struct Span(
    /// Starting position.
    pub Pos,
    /// End position (inclusive).
    pub Pos,
);

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.0, self.1)
    }
}

/// Various kinds of tokens.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// Unit literal.
    UnitLit,
    /// Integer literal.
    IntLit(i64),
    /// Floating-point literal.
    FloatLit(f64),
    /// Character literal.
    CharLit(char),
    /// String literal.
    StrLit(String),

    /// Alphabetic/symbolic name.
    Name(String),

    /// `(` (left parenthesis).
    Lp,
    /// `)` (right parenthesis).
    Rp,
    /// `[` (left bracket).
    Lb,
    /// `]` (right bracket).
    Rb,
    /// `{` (left curly brace).
    Lc,
    /// `}` (right curly brace).
    Rc,
    /// `;`.
    Semicolon,
}

/// Token of Lynx source.
#[derive(Debug)]
pub struct Token(
    /// Kind of the token.
    pub TokenKind,
    /// Position in the source.
    pub Span,
);

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}@{}", self.0, self.1)
    }
}
