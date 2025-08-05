use std::{collections::HashMap, iter::Peekable, str::Chars};

use crate::token::Token;

#[derive(Debug)]
pub enum LexerErr {
    MalformedNum,
    UnexpectedChar(char),
    UnterminatedStr,
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    keywords_table: HashMap<&'a str, Token>,
}

fn build_keywords_table() -> HashMap<&'static str, Token> {
    let mut keywords_table = HashMap::new();
    keywords_table.insert("async"   , Token::Async);
    keywords_table.insert("await"   , Token::Await);
    keywords_table.insert("break"   , Token::Break);
    keywords_table.insert("continue", Token::Continue);
    keywords_table.insert("do"      , Token::Do);
    keywords_table.insert("else"    , Token::Else);
    keywords_table.insert("fn"      , Token::Fn);
    keywords_table.insert("for"     , Token::For);
    keywords_table.insert("if"      , Token::If);
    keywords_table.insert("import"  , Token::Import);
    keywords_table.insert("in"      , Token::In);
    keywords_table.insert("match"   , Token::Match);
    keywords_table.insert("mod"     , Token::Mod);
    keywords_table.insert("not"     , Token::Not);
    keywords_table.insert("return"  , Token::Return);
    keywords_table.insert("then"    , Token::Then);
    keywords_table.insert("type"    , Token::Type);
    keywords_table.insert("var"     , Token::Var);
    keywords_table.insert("while"   , Token::While);
    keywords_table.insert("with"    , Token::With);
    keywords_table
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            chars: src.chars().peekable(),
            keywords_table: build_keywords_table(),
        }
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

    fn lex_str_literal(&mut self) -> Result<Token, LexerErr> {
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

    fn lex_num_literal(&mut self) -> Result<Token, LexerErr> {
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

        match self.keywords_table.get(name.as_str()) {
            Some(keyword_token) => Ok(keyword_token.to_owned()),
            None => Ok(Token::Id(name)),
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexerErr> {
        self.skip_ws();

        match self.chars.peek() {
            Some('#') => {
                self.skip_comment();
                self.next_token()
            }
            Some('"') => {
                self.lex_str_literal()
            }
            Some(&c) if c.is_ascii_digit() => {
                self.lex_num_literal()
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

    pub fn tokens(&mut self) -> Result<Vec<Token>, LexerErr> {
        let mut tokens = Vec::new();
        loop {
            match self.next_token() {
                Ok(Token::EOF) => {
                    tokens.push(Token::EOF);
                    break;
                }
                Ok(token) => {
                    tokens.push(token);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(tokens)
    } 
}
