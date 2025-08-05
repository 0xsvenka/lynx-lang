use crate::token::Token;

use std::iter::Peekable;

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
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
}

pub struct Parser<'a> {
    tokens: Peekable<std::slice::Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens: tokens.iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        let mut exprs = Vec::new();
        while self.tokens.peek() != Some(&&Token::EOF) {
            exprs.push(self.expr());
        }
        exprs
    }

    fn expr(&mut self) -> Expr {
        match self.tokens.peek() {
            Some(Token::If) => self.if_expr(),
            Some(Token::While) => self.while_expr(),
            Some(Token::For) => self.for_expr(),
            Some(Token::Lp) => self.block_expr(),
            _ => self.binary_expr(),
        }
    }

    fn if_expr(&mut self) -> Expr {
        self.consume(Token::If, "Expected 'if'");
        let condition = Box::new(self.expr());
        let then_branch = Box::new(self.block_expr());

        let else_branch = if self.match_token(Token::Else) {
            Some(Box::new(self.expr()))
        } else {
            None
        };

        Expr::If {
            condition,
            then_branch,
            else_branch,
        }
    }

    fn while_expr(&mut self) -> Expr {
        self.consume(Token::While, "Expected 'while'");
        let condition = Box::new(self.expr());
        let body = Box::new(self.block_expr());
        Expr::While { condition, body }
    }

    fn for_expr(&mut self) -> Expr {
        self.consume(Token::For, "Expected 'for'");
        self.consume(Token::LParen, "Expected '(' after 'for'");

        let init = Box::new(self.expr());
        self.consume(Token::Semicolon, "Expected ';' after init");
        let cond = Box::new(self.expr());
        self.consume(Token::Semicolon, "Expected ';' after condition");
        let update = Box::new(self.expr());
        self.consume(Token::RParen, "Expected ')' after for clauses");

        let body = Box::new(self.block_expr());

        Expr::For {
            init,
            cond,
            update,
            body,
        }
    }

    fn block_expr(&mut self) -> Expr {
        self.consume(Token::LBrace, "Expected '{' before block");
        let mut exprs = Vec::new();

        while self.tokens.peek() != Some(&&Token::RBrace) && self.tokens.peek() != Some(&&Token::EOF) {
            exprs.push(self.expr());
            // Automatically insert semicolons between expressions in block
            if self.tokens.peek() != Some(&&Token::RBrace) {
                self.consume(Token::Semicolon, "Expected ';' after expression");
            }
        }

        self.consume(Token::RBrace, "Expected '}' after block");
        Expr::Block { exprs }
    }

    fn binary_expr(&mut self) -> Expr {
        let mut expr = self.term();

        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Plus | Token::Minus | Token::Multiply | Token::Divide => {
                    let op = match self.tokens.next().unwrap() {
                        Token::Plus => BinaryOp::Add,
                        Token::Minus => BinaryOp::Subtract,
                        Token::Multiply => BinaryOp::Multiply,
                        Token::Divide => BinaryOp::Divide,
                        _ => unreachable!(),
                    };
                    let right = Box::new(self.term());
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op,
                        right,
                    };
                }
                _ => break,
            }
        }

        expr
    }

    fn term(&mut self) -> Expr {
        self.factor()
    }

    fn factor(&mut self) -> Expr {
        match self.tokens.peek() {
            Some(Token::Number(n)) => {
                let val = *n;
                self.tokens.next();
                Expr::Number(val)
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.tokens.next();
                Expr::Variable(name)
            }
            Some(Token::LParen) => {
                self.tokens.next();
                let expr = self.expr();
                self.consume(Token::RParen, "Expected ')' after expression");
                expr
            }
            Some(Token::Minus) => {
                self.tokens.next();
                let expr = Box::new(self.factor());
                Expr::Unary {
                    op: UnaryOp::Negate,
                    expr,
                }
            }
            _ => panic!("Unexpected token"),
        }
    }

    // Helper methods
    fn consume(&mut self, expected: Token, error_msg: &str) {
        if self.tokens.peek() == Some(&&expected) {
            self.tokens.next();
        } else {
            panic!("{}", error_msg);
        }
    }

    fn match_token(&mut self, expected: Token) -> bool {
        if self.tokens.peek() == Some(&&expected) {
            self.tokens.next();
            true
        } else {
            false
        }
    }
}