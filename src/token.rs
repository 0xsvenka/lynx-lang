use std::fmt;

/// Position of a character in Lynx source code.
#[derive(Debug, Clone, Copy)]
pub struct Pos(
    /// Line number, starting from `1`.
    pub usize,
    /// Column number, starting from `1`.
    ///
    /// However, note that for a blank line [`Token`]
    /// (whose [`TokenKind`] is [`ExprEnd`](TokenKind::ExprEnd)),
    /// this field denotes the last column of the line, e.g. `3`
    /// if the line contains only three whitespace characters and
    /// nothing else; therefore, this field is `0` if and only if
    /// the line contains no character at all.
    pub usize,
);

impl fmt::Display for Pos {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

/// Position of a span of text in Lynx source code.
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
    /// Identifier.
    Id(
        /// Name of the identifier.
        String,
    ),

    /// Infix operator.
    Op(
        /// Name of the operator.
        String,
    ),

    /// Interger literal.
    IntLit(
        /// Value of the literal.
        i64,
    ),

    /// Floating-point number literal.
    FloatLit(
        /// Value of the literal.
        f64,
    ),

    /// Character literal.
    CharLit(
        /// Value of the literal.
        char,
    ),

    /// String literal.
    StrLit(
        /// Value of the literal.
        String,
    ),

    // Alphabetic keywords
    /// Keyword `ctor`.
    Ctor,
    /// Keyword `import`.
    Import,
    /// Keyword `_`.
    Underscore,

    // Symbolic keywords
    /// Keyword `:`.
    Colon,
    /// Keyword `::`.
    DoubleColon,
    /// Keyword `.`.
    Dot,
    /// Keyword `->`.
    Arrow,
    /// Keyword `?`.
    Question,
    /// Keyword `~`.
    Tilde,
    /// Keyword `|`.
    Pipe,
    /// Keyword `@`.
    At,
    /// Keyword `=>`.
    FatArrow,
    /// Keyword `=`.
    Eq,

    // Separators
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
    /// `,`.
    Comma,
    /// End of expression, i.e. `;` or blank line.
    ExprEnd,
}

/// Token of Lynx source code.
#[derive(Debug)]
pub struct Token(
    /// Kind of the token.
    pub TokenKind,
    /// Position in the source code.
    pub Span,
);

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}@{}", self.0, self.1)
    }
}
