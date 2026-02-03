use std::fmt::Display;

use crate::token::Span;

#[derive(Debug)]
pub enum Expr {
    Atom(AtomType, Span),
    App(Box<Expr>, Box<Expr>, Span),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
pub enum AtomType {
    UnitLit,
    IntLit(i64),
    FloatLit(f64),
    CharLit(char),
    StrLit(String),

    Wildcard,

    Name(String),
}
