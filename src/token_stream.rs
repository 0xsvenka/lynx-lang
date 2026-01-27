use crate::error::Error;
use crate::token::Token;

/// TokenStream wraps a [Vec] of [Token]s,
/// providing capabilities for peeking, expecting, etc.
pub struct TokenStream {
    pub buffer: Vec<Token>,
    pub pos: usize,
}

impl TokenStream {
    /// Creates a [TokenStream] from a [Vec<Token>].
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            buffer: tokens,
            pos: 0,
        }
    }

    /// Peeks at the `n`th token ahead.
    pub fn peek(&self, n: usize) -> Option<&Token> {
        self.buffer.get(self.pos + n)
    }

    /// Gets the next token, consuming it.
    pub fn next(&mut self) -> Option<&Token> {
        if self.pos < self.buffer.len() {
            let tok = &self.buffer[self.pos];
            self.pos += 1;
            Some(tok)
        } else {
            None
        }
    }

    /// Expects the next token to satisfy a predicate, or returns an error.
    /// Useful for parser: e.g. expect(|t| matches!(t.kind, TokenKind::Lp), "expected '('")
    pub fn expect<F>(&mut self, pred: F, err: Error) -> Result<&Token, Error>
    where
        F: Fn(&Token) -> bool,
    {
        match self.next() {
            Some(tok) if pred(tok) => Ok(tok),
            Some(_) | None => Err(err),
        }
    }
}
