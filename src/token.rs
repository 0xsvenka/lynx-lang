#[derive(Debug, PartialEq)]
pub enum Token {
    // Keywords
    Async,
    Await,
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
    Then,
    Type,
    While,
    With,

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
    Bind,           // =
    Assign,         // :=
    Range,          // ..
    
    // Separators
    Lp,             // (
    Rp,             // )
    Lb,             // [
    Rb,             // ]
    Lc,             // {
    Rc,             // }
    Colon,          // :
    Comma,          // ,
    ExprEnd,        // ; or EOL         
    Dot,            // .
    Ellipsis,       // ...
    Underscore,     // _
    Tilde,          // ~
    Arrow,          // ->
    FatArrow,       // =>

    // Literals
    StrLiteral(String),
    NumLiteral(i64),

    Id(String),

    EOF,
}
