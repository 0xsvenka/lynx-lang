#[derive(Debug)]
pub enum Expr {
    Num(i64),
    Str(String),

    Id(String),

    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Prefix {
        op: PrefixOp,
        expr: Box<Expr>,
    },
    Suffix {
        expr: Box<Expr>,
        op: SuffixOp,
    },

    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
    While {
        condition: Box<Expr>,
        body: Box<Expr>,
    },
    For {
        init: Box<Expr>,
        cond: Box<Expr>,
        update: Box<Expr>,
        body: Box<Expr>,
    },
    
    Block {
        exprs: Vec<Expr>,
    },
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
    And,
    Or,
    Intersection,
    Union,
    Concat,
    Bind,
    Assign,
    Range,
    Pipeline,
}

#[derive(Debug)]
pub enum PrefixOp {
    Neg,
    Ellipsis,
}

#[derive(Debug)]
pub enum SuffixOp {
    Ellipsis,
}
