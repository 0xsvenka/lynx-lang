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
    // Keywords
    Break,
    Continue,
    Do,
    Else,
    Fn,
    For,
    If,
    Import,
    In,
    Match,
    Mod,
    Not,
    Return,
    Self_,          // Avoid confusion with Rust `Self`
    Then,
    Type,
    Var,
    While,
    With,

    // Separators
    Lp,             // (
    Rp,             // )
    Lb,             // [
    Rb,             // ]
    Lc,             // {
    Rc,             // }
    Colon,          // :
    DoubleColon,    // ::
    Comma,          // ,
    ExprEnd,        // ; or EOL
    ExprContinue,   // \
    Dot,            // .
    Underscore,     // _
    Arrow,          // ->
    FatArrow,       // =>
    Undefined,      // ?

    // Operators
    Add,            // +
    Sub,            // -
    Mul,            // *
    Div,            // /
    Exp,            // ^
    Eq,             // ==
    Ne,             // !=
    Gt,             // >
    Lt,             // <
    Ge,             // >=
    Le,             // <=
    And,            // &&
    Or,             // ||
    Intersection,   // &
    Union,          // |
    Concat,         // ++
    Bind,           // =
    Assign,         // :=
    Range,          // ..
    Ellipsis,       // ...
    At,             // @
    Pipeline,       // $
    Tilde,          // ~
    
    // Literals
    StrLiteral(String),
    NumLiteral(i64),    // TODO: Support more kinds of number literals

    Id(String),
}

#[derive(Debug)]
pub struct Token(pub TokenKind, pub Span);
