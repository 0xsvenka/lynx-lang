use std::{collections::HashMap, iter::Peekable, str::Chars};

use crate::{error::Error, token::Token};

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    keywords_table: HashMap<&'a str, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            chars: src.chars().peekable(),

            keywords_table: HashMap::from([
                ("break"   , Token::Break),
                ("continue", Token::Continue),
                ("do"      , Token::Do),
                ("else"    , Token::Else),
                ("fn"      , Token::Fn),
                ("for"     , Token::For),
                ("if"      , Token::If),
                ("import"  , Token::Import),
                ("in"      , Token::In),
                ("match"   , Token::Match),
                ("mod"     , Token::Mod),
                ("not"     , Token::Not),
                ("return"  , Token::Return),
                ("self"    , Token::Self_),
                ("then"    , Token::Then),
                ("type"    , Token::Type),
                ("var"     , Token::Var),
                ("while"   , Token::While),
                ("with"    , Token::With),
            ]),
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

    fn lex_str_literal(&mut self) -> Result<Token, Error> {
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

        Err(Error::UnterminatedStr) 
    }

    fn lex_num_literal(&mut self) -> Result<Token, Error> {
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
            Err(Error::InvalidNumFormat)
        }
    }

    fn lex_id_or_keyword(&mut self) -> Token {
        let mut name = String::new();
        while let Some(&c) = self.chars.peek() {
            // '!' is allowed in identifiers (though not as the first character)
            if !(c.is_alphanumeric() || c == '_' || c == '!') {
                break;
            }
            name.push(c);
            self.chars.next();
        }

        match self.keywords_table.get(name.as_str()) {
            Some(keyword_token) => keyword_token.to_owned(),
            None => Token::Id(name),
        }
    }

    fn lex_others(&mut self, c: char) -> Result<Token, Error> {
        match c {
            // Lex separators & operators
            '(' => {
                self.chars.next();
                Ok(Token::Lp)
            }
            ')' => {
                self.chars.next();
                Ok(Token::Rp)
            }
            '[' => {
                self.chars.next();
                Ok(Token::Lb)
            }
            ']' => {
                self.chars.next();
                Ok(Token::Rb)
            }
            '{' => {
                self.chars.next();
                Ok(Token::Lc)
            }
            '}' => {
                self.chars.next();
                Ok(Token::Rc)
            }
            ':' => {
                self.chars.next();
                match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        Ok(Token::Assign)
                    }
                    Some(':') => {
                        self.chars.next();
                        Ok(Token::DoubleColon)
                    }
                    _ => Ok(Token::Colon)
                }
            }
            ',' => {
                self.chars.next();
                Ok(Token::Comma)
            }
            ';' | '\n' => {
                self.chars.next();
                Ok(Token::ExprEnd)
            }
            '.' => {
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
            '_' => {
                self.chars.next();
                Ok(Token::Underscore)
            }
            '~' => {
                self.chars.next();
                Ok(Token::Tilde)
            }
            '@' => {
                self.chars.next();
                Ok(Token::At)
            }
            '+' => {
                self.chars.next();
                match self.chars.peek() {
                    Some('+') => {
                        self.chars.next();
                        Ok(Token::Concat)
                    }
                    _ => Ok(Token::Add)
                }
            }
            '-' => {
                self.chars.next();
                match self.chars.peek() {
                    Some('>') => {
                        self.chars.next();
                        Ok(Token::Arrow)
                    }
                    _ => Ok(Token::Sub)
                }
            }
            '*' => {
                self.chars.next();
                Ok(Token::Mul)

            }
            '/' => {
                self.chars.next();
                Ok(Token::Div)
            }
            '^' => {
                self.chars.next();
                Ok(Token::Exp)
            }
            '=' => {
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
            '!' => {
                self.chars.next();
                match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        Ok(Token::Ne)
                    }
                    _ => Err(Error::UnsupportedOperator("!"))
                }
            }
            '>' => {
                self.chars.next();
                match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        Ok(Token::Ge)
                    }
                    _ => Ok(Token::Gt)
                }
            }
            '<' => {
                self.chars.next();
                match self.chars.peek() {
                    Some('>') => {
                        self.chars.next();
                        Ok(Token::Le)
                    }
                    _ => Ok(Token::Lt)
                }
            }
            '&' => {
                self.chars.next();
                match self.chars.peek() {
                    Some('&') => {
                        self.chars.next();
                        Ok(Token::And)
                    }
                    _ => Ok(Token::Intersection)
                }
            }
            '|' => {
                self.chars.next();
                match self.chars.peek() {
                    Some('|') => {
                        self.chars.next();
                        Ok(Token::Or)
                    }
                    _ => Ok(Token::Union)
                }
            }
            '$' => {
                self.chars.next();
                Ok(Token::Pipeline)
            }

            other => {
                Err(Error::UnexpectedChar(other))
            }
        }
    }

    fn next_token(&mut self) -> Result<Token, Error> {
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
                Ok(self.lex_id_or_keyword())
            }
            Some(&c) => {
                self.lex_others(c)
            }
            None => Ok(Token::EOF)
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next_token();
        match next {
            Ok(Token::EOF) => None,
            _ => Some(next),
        }
    }
}
