use std::{iter::Peekable, str::Chars};

use crate::token::Token;

#[derive(Debug)]
pub enum LexerErr {
    MalformedNum,
    UnexpectedChar(char),
    UnterminatedStr,
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { chars: src.chars().peekable() }
    }

    fn skip_ws(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if !c.is_whitespace() || c == '\n' {
                break;
            }
            self.chars.next();
        }
    }

    fn skip_comment(&mut self) {
        while let Some(c) = self.chars.next() {
            if c == '\n' {
                break;
            }
        }
    }

    fn lex_str(&mut self) -> Result<Token, LexerErr> {
        let mut s = String::new();
        self.chars.next();    // Skip opening quote

        while let Some(c) = self.chars.next() {
            match c {
                '"' => return Ok(Token::StrLiteral(s)),    // Closing quote

                '\\' => {   // Escape sequence
                    if let Some(escaped) = self.chars.next() {
                        s.push(match escaped {
                            'n' => '\n',
                            't' => '\t',
                            // TODO: support more escape sequences...
                            _ => escaped,
                        });
                    }
                }

                _ => s.push(c),
            }
        }

        Err(LexerErr::UnterminatedStr) 
    }

    fn lex_num(&mut self) -> Result<Token, LexerErr> {
        let mut num_str = String::new();
        while let Some(&c) = self.chars.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            num_str.push(c);
            self.chars.next();
        }
        if let Ok(num) = num_str.parse() {
            Ok(Token::NumLiteral(num))
        } else {
            Err(LexerErr::MalformedNum)
        }
    }

    fn lex_id_or_keyword(&mut self) -> Result<Token, LexerErr> {
        let mut name = String::new();
        while let Some(&c) = self.chars.peek() {
            if !(c.is_alphanumeric() || c == '_') {
                break;
            }
            name.push(c);
            self.chars.next();
        }

        let token = match name.as_str() {
            "async"     => Token::Async,
            "await"     => Token::Await,
            "break"     => Token::Break,
            "continue"  => Token::Continue,
            "do"        => Token::Do,
            "else"      => Token::Else,
            "fn"        => Token::Fn,
            "for"       => Token::For,
            "if"        => Token::If,
            "import"    => Token::Import,
            "in"        => Token::In,
            "match"     => Token::Match,
            "mod"       => Token::Mod,
            "not"       => Token::Not,
            "return"    => Token::Return,
            "then"      => Token::Then,
            "type"      => Token::Type,
            "var"       => Token::Var,
            "while"     => Token::While,
            "with"      => Token::With,

            _           => Token::Id(name),
        };
        Ok(token)
    }

    pub fn next_token(&mut self) -> Result<Token, LexerErr> {
        self.skip_ws();

        match self.chars.peek() {
            Some('#') => {
                self.skip_comment();
                self.next_token()
            }
            Some('"') => {
                self.lex_str()
            }
            Some(&c) if c.is_ascii_digit() => {
                self.lex_num()
            }
            Some(&c) if c.is_alphabetic() || c == '_' => {
                self.lex_id_or_keyword()
            }
            
            // Lex operators & separators
            Some('(') => {
                self.chars.next();
                Ok(Token::Lp)
            }
            Some(')') => {
                self.chars.next();
                Ok(Token::Rp)
            }
            Some('[') => {
                self.chars.next();
                Ok(Token::Lb)
            }
            Some(']') => {
                self.chars.next();
                Ok(Token::Rb)
            }
            Some('{') => {
                self.chars.next();
                Ok(Token::Lc)
            }
            Some('}') => {
                self.chars.next();
                Ok(Token::Rc)
            }
            Some(':') => {
                self.chars.next();
                match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        Ok(Token::Assign)
                    }
                    _ => Ok(Token::Colon)
                }
            }
            Some(',') => {
                self.chars.next();
                Ok(Token::Comma)
            }
            Some(';') | Some('\n') => {
                self.chars.next();
                Ok(Token::ExprEnd)
            }
            Some('.') => {
                self.chars.next();
                match self.chars.peek() {
                    Some('.') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('.') => {
                                self.chars.next();
                                Ok(Token::Ellipsis)
                            }
                            _ => Ok(Token::Range)
                        }
                    }
                    _ => Ok(Token::Dot)
                }
            }
            Some('_') => {
                self.chars.next();
                Ok(Token::Underscore)
            }
            Some('~') => {
                self.chars.next();
                Ok(Token::Tilde)
            }
            Some('+') => {
                self.chars.next();
                Ok(Token::Add)
            }
            Some('-') => {
                self.chars.next();
                match self.chars.peek() {
                    Some('>') => {
                        self.chars.next();
                        Ok(Token::Arrow)
                    }
                    _ => Ok(Token::Sub)
                }
            }
            Some('*') => {
                self.chars.next();
                Ok(Token::Mul)

            }
            Some('/') => {
                self.chars.next();
                Ok(Token::Div)
            }
            Some('^') => {
                self.chars.next();
                Ok(Token::Exp)
            }
            Some('=') => {
                self.chars.next();
                match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        Ok(Token::Eq)
                    }
                    Some('>') => {
                        self.chars.next();
                        Ok(Token::FatArrow)
                    }
                    _ => Ok(Token::Bind)
                }
            }
            Some('!') => {
                self.chars.next();
                match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        Ok(Token::Ne)
                    }
                    _ => Err(LexerErr::UnexpectedChar('!'))
                }
            }
            Some('>') => {
                self.chars.next();
                match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        Ok(Token::Ge)
                    }
                    _ => Ok(Token::Gt)
                }
            }
            Some('<') => {
                self.chars.next();
                match self.chars.peek() {
                    Some('>') => {
                        self.chars.next();
                        Ok(Token::Le)
                    }
                    _ => Ok(Token::Lt)
                }
            }
            Some('&') => {
                self.chars.next();
                match self.chars.peek() {
                    Some('&') => {
                        self.chars.next();
                        Ok(Token::And)
                    }
                    _ => Ok(Token::Intersection)
                }
            }
            Some('|') => {
                self.chars.next();
                match self.chars.peek() {
                    Some('|') => {
                        self.chars.next();
                        Ok(Token::Or)
                    }
                    _ => Ok(Token::Union)
                }
            }

            Some(&c) => {
                Err(LexerErr::UnexpectedChar(c))
            }
            None => Ok(Token::EOF)
        }
    }
}
