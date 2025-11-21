use std::fmt;

/// A character's position in Lynx source code.
#[derive(Debug, Clone, Copy)]
pub struct Pos(
    /// Line number, starting from `1`.
    pub usize,
    /// Column number, starting from `1`.
    pub usize,
);

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

/// Various kinds of tokens.
#[derive(Debug, Clone)]
pub enum TokenKind {
    /// Identifiers.
    Id(
        /// Name of the identifier.
        String,
    ),

    /// Infix operators.
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
    /// Keyword `=>`.
    FatArrow,
    /// Keyword `=`.
    Bind,
    /// Keyword `@`.
    At,
    /// Keyword `|`.
    Pipe,
    /// Keyword `#`.
    Hash,
    /// Keyword `%`.
    Percent,
    /// Keyword `~`.
    Tilde,
    /// Keyword `%~`.
    PercentTilde,

    // Separators
    /// Keyword `(` (left parenthesis).
    Lp,
    /// Keyword `)` (right parenthesis).
    Rp,
    /// Keyword `[` (left bracket).
    Lb,
    /// Keyword `]` (right bracket).
    Rb,
    /// Keyword `{` (left curly brace).
    Lc,
    /// Keyword `}` (right curly brace).
    Rc,
    /// Keyword `,`.
    Comma,
    /// End of expression, i.e. `;` or blank line.
    ExprEnd,
}

/// A token of Lynx source code.
#[derive(Debug, Clone)]
pub struct Token(
    /// Kind of the token.
    pub TokenKind,
    /// Starting position of the token.
    pub Pos,
    /// End position of the token (inclusive).
    pub Pos,
);
