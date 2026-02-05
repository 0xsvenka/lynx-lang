use std::fmt::Display;

use crate::token::Span;

#[derive(Debug)]
pub enum Expr {
    Atom(AtomKind, Span),
    App(Box<Expr>, Box<Expr>, Span),
    Block(Vec<Expr>, Span),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Atom(atom_type, _) => write!(f, "{}", atom_type),
            Expr::App(func, arg, _) => write!(f, "({} {})", func, arg),
            Expr::Block(exprs, _) => {
                write!(f, "[")?;
                for expr in exprs {
                    write!(f, "{} ", expr)?;
                }
                write!(f, "]")
            }
        }
    }
}

#[derive(Debug)]
pub enum AtomKind {
    UnitLit,
    IntLit(i64),
    FloatLit(f64),
    CharLit(char),
    StrLit(String),

    Wildcard,

    Name(String),
}

impl Display for AtomKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AtomKind::UnitLit => write!(f, "()"),
            AtomKind::IntLit(value) => write!(f, "{:?}", value),
            AtomKind::FloatLit(value) => write!(f, "{:?}", value),
            AtomKind::CharLit(value) => write!(f, "{:?}", value),
            AtomKind::StrLit(value) => write!(f, "{:?}", value),
            AtomKind::Wildcard => write!(f, "_"),
            AtomKind::Name(name) => write!(f, "{}", name),
        }
    }
}
