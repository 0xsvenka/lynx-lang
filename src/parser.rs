use std::iter::Peekable;
use std::slice::Iter;

use crate::{error::Error, expr::Expr, lexer::Lexer, token::Token};

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, >>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens: tokens.iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;

        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Plus | Token::Minus => {
                    let op = match self.tokens.next().unwrap() {
                        Token::Plus => Op::Add,
                        Token::Minus => Op::Subtract,
                        _ => unreachable!(),
                    };
                    let right = self.parse_factor()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;

        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Multiply | Token::Divide => {
                    let op = match self.tokens.next().unwrap() {
                        Token::Multiply => Op::Multiply,
                        Token::Divide => Op::Divide,
                        _ => unreachable!(),
                    };
                    let right = self.parse_primary()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.tokens.next() {
            Some(Token::Number(n)) => Ok(Expr::Number(*n)),
            Some(Token::LParen) => {
                let expr = self.parse_expr()?;
                match self.tokens.next() {
                    Some(Token::RParen) => Ok(expr),
                    _ => Err("Expected ')' after expression".to_string()),
                }
            }
            Some(t) => Err(format!("Unexpected token: {:?}", t)),
            None => Err("Unexpected end of input".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_expression() {
        let tokens = vec![
            Token::Number(2),
            Token::Plus,
            Token::Number(3),
            Token::Multiply,
            Token::Number(4),
            Token::EOF,
        ];
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse().unwrap();
        
        assert_eq!(
            expr,
            Expr::BinaryOp {
                left: Box::new(Expr::Number(2)),
                op: Op::Add,
                right: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Number(3)),
                    op: Op::Multiply,
                    right: Box::new(Expr::Number(4)),
                }),
            }
        );
    }

    #[test]
    fn test_parentheses() {
        let tokens = vec![
            Token::LParen,
            Token::Number(2),
            Token::Plus,
            Token::Number(3),
            Token::RParen,
            Token::Multiply,
            Token::Number(4),
            Token::EOF,
        ];
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse().unwrap();
        
        assert_eq!(
            expr,
            Expr::BinaryOp {
                left: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Number(2)),
                    op: Op::Add,
                    right: Box::new(Expr::Number(3)),
                }),
                op: Op::Multiply,
                right: Box::new(Expr::Number(4)),
            }
        );
    }
}