#[derive(Debug, PartialEq, Clone)]
pub enum Token {
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
    Dot,            // .
    Ellipsis,       // ...
    Underscore,     // _
    Tilde,          // ~
    At,             // @
    Arrow,          // ->
    FatArrow,       // =>

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
    Concat,          // ++
    Bind,           // =
    Assign,         // :=
    Range,          // ..
    Pipeline,       // $
    
    // Literals
    StrLiteral(String),
    NumLiteral(i64),

    Id(String),

    EOF,
}
