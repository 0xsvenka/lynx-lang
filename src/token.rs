#[derive(Debug, PartialEq, Clone)]
pub enum Token {    // TODO: Make each token remember its position in source code
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
    NumLiteral(i64),

    Id(String),

    EOF,
}
