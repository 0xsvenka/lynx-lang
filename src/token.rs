use std::fmt;

#[derive(Debug, Clone)]
pub struct Pos(pub usize, pub usize);   // (line, column)

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

#[derive(Debug)]
pub struct Span(pub Pos, pub Pos);      // (start_pos, end_pos)

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.0, self.1)
    }
}

#[derive(Debug, Clone)]
pub enum TokenKind {

    /// Identifier, which falls into either of the two categories:
    /// alphabetic identifier, starting with an alphabetic character or `_`,
    /// possibly containing alphanumeric characters, `'`, `!` and `_`;
    /// symbolic identifier, consisting of symbolic characters
    /// `~`, `` ` ``, `!`, `@`, `$`, `%`, `^`, `&`, `*`, `_`, `-`, `+`, `=`,
    /// `|`, `:`, `<`, `>`, `.`, `?`, and `/`
    Id(String),

    /// Interger literal
    IntLit(i64),

    /// Floating-point number literal
    FloatLit(f64),

    /// Character literal
    CharLit(char),

    /// String literal
    StrLit(String),

    // Keywords

    // Alphabetic keywords
    /// The keyword `case`
    Case,
    /// The keyword `import`
    Import,
    /// The keyword `of`
    Of,

    // Symbolic keywords
    /// `:`
    Colon,
    /// `::`
    DoubleColon,
    /// `.`
    Dot,
    /// `_`
    Underscore,
    /// `->`
    Arrow,
    /// `=>`
    FatArrow,
    /// `=`
    Bind,
    /// `@`
    At,
    /// `|`
    Pipe,
    /// `%`
    Percent,
    /// `~`
    Tilde,
    /// `%~`
    PercentTilde,
    /// `(` (left parenthesis)
    Lp,
    /// `)` (right parenthesis) 
    Rp,
    /// `(|`
    LpPipe,
    /// `|)`
    PipeRp,
    /// `[` (left bracket)
    Lb,
    /// `]` (right bracket)
    Rb,
    /// `[|`
    LbPipe,
    /// `|]`
    PipeRb,
    /// `{` (left curly brace)
    Lc,
    /// `}` (right curly brace)
    Rc,
    /// `{|`
    LcPipe,
    /// `|}`
    PipeRc,
    /// `,`
    Comma,
    /// `;` or `EOL`
    ExprEnd,
    /// `\`
    ExprContinue,
}

#[derive(Debug)]
pub struct Token(pub TokenKind, pub Span);
