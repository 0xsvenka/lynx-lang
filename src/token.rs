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

    /// An identifier starting with a lowercase letter or `_`,
    /// possibly containing alphanumeric characters, `'`, `!` or `_`.
    Id(String),

    /// A constructor identifier starting with an uppercase letter,
    /// possibly containing alphanumeric characters, `'`, `!` or `_`.
    ConId(String),

    /// Interger literal
    Int(i64),

    /// Floating-point number literal
    Float(f64),

    /// Character literal
    Char(char),

    /// String literal
    Str(String),

    /// ## Keywords
    /// The keyword `case`
    Case,
    /// The keyword `fn`
    Fn,
    /// The keyword `import`
    Import,
    /// The keyword `of`
    Of,
    /// The keyword `self`
    SelfLower,
    /// The keyword `Self`
    SelfUpper,
    /// The keyword `type`
    Type,
    /// The keyword `var`
    Var,
    // Symbolic keywords are also keywords
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
    /// `:=`
    Assign,
    /// `@`
    At,
    /// `|`
    Pipe,

    /// ## Separators
    /// Left parenthesis `(`
    Lp,
    /// Right parenthesis `)`
    Rp,
    /// Left bracket `[`
    Lb,
    /// Right bracket `]`
    Rb,
    /// Left curly brace `{`
    Lc,
    /// Right curly brace `}`
    Rc,
    /// `,`
    Comma,
    /// `;` or `EOL`
    ExprEnd,
    /// `\`
    ExprContinue,
}

#[derive(Debug)]
pub struct Token(pub TokenKind, pub Span);
