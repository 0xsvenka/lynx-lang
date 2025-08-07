#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Variable(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
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
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
}
