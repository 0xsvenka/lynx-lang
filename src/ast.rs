use std::fmt::Display;

use crate::token::Span;

#[derive(Debug)]
pub enum Expr {
    Atom(AtomType, Span),
    App(Box<Expr>, Vec<Expr>, Span),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
pub enum AtomType {
    Id(String),
    Op(String),
    IntLit(i64),
    FloatLit(f64),
    CharLit(f64),
    StrLit(String),
}
