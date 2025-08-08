use std::{collections::HashMap, iter::Peekable, str::Chars};

use crate::{error::Error, token::{Pos, Token}};

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    pos: Pos,

    keywords_table: HashMap<&'a str, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            chars: src.chars().peekable(),
            pos: Pos(1, 1),

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

    #[inline]
    fn advance_in_same_line(&mut self) {
        self.pos.1 += 1;
        self.chars.next();
    }

    #[inline]
    fn advance_to_next_line(&mut self) {
        self.pos.0 += 1;
        self.pos.1 = 0;     // We haven't come to the first character of the next line yet
        self.chars.next();
    }

    fn skip_ws(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if !c.is_whitespace() || c == '\n' {
                break;
            }
            self.advance_in_same_line();
        }
    }

    fn skip_comment(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c == '\n' {
                self.advance_to_next_line();
                break;
            } else {
                self.advance_in_same_line();
            }
        }
    }

    fn lex_str_literal(&mut self) -> Result<Token, Error> {
        let mut s = String::new();
        self.advance_in_same_line();    // Skip opening quote
        let start_pos = self.pos.to_owned();

        while let Some(&c) = self.chars.peek() {
            match c {
                '"' => {    // Closing quote
                    self.advance_in_same_line();
                    return Ok(Token::StrLiteral(s));
                }

                '\\' => {   // Escape sequence
                    self.advance_in_same_line();
                    match self.chars.peek() {
                        Some('n') => {
                            self.advance_in_same_line();
                            s.push('\n');
                        }
                        Some('t') => {
                            self.advance_in_same_line();
                            s.push('\t');
                        }
                        // TODO: support more escape sequences...

                        Some('\n') => {
                            self.advance_to_next_line();
                            s.push('\n');
                        }
                        Some(&escaped) => {
                            self.advance_in_same_line();
                            s.push(escaped);
                        }
                        None => {}
                    }
                }

                // String literals can occupy multiple lines
                '\n' => {
                    self.advance_to_next_line();
                    s.push('\n');
                }
                _ => {
                    self.advance_in_same_line();
                    s.push(c);
                }
            }
        }

        Err(Error::UnterminatedStr(start_pos)) 
    }

    fn lex_num_literal(&mut self) -> Result<Token, Error> {
        let mut num_str = String::new();
        while let Some(&c) = self.chars.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            num_str.push(c);
            self.advance_in_same_line();
        }
        if let Ok(num) = num_str.parse() {
            Ok(Token::NumLiteral(num))
        } else {
            Err(Error::InvalidNumFormat(self.pos.to_owned()))
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
            self.advance_in_same_line();
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
                self.advance_in_same_line();
                Ok(Token::Lp)
            }
            ')' => {
                self.advance_in_same_line();
                Ok(Token::Rp)
            }
            '[' => {
                self.advance_in_same_line();
                Ok(Token::Lb)
            }
            ']' => {
                self.advance_in_same_line();
                Ok(Token::Rb)
            }
            '{' => {
                self.advance_in_same_line();
                Ok(Token::Lc)
            }
            '}' => {
                self.advance_in_same_line();
                Ok(Token::Rc)
            }
            ':' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('=') => {
                        self.advance_in_same_line();
                        Ok(Token::Assign)
                    }
                    Some(':') => {
                        self.advance_in_same_line();
                        Ok(Token::DoubleColon)
                    }
                    _ => Ok(Token::Colon)
                }
            }
            ',' => {
                self.advance_in_same_line();
                Ok(Token::Comma)
            }
            ';' => {
                self.advance_in_same_line();
                Ok(Token::ExprEnd)
            }
            '\n' => {
                self.advance_to_next_line();
                Ok(Token::ExprEnd)
            }
            '\\' => {
                self.advance_in_same_line();
                Ok(Token::ExprContinue)
            }
            '.' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('.') => {
                        self.advance_in_same_line();
                        match self.chars.peek() {
                            Some('.') => {
                                self.advance_in_same_line();
                                Ok(Token::Ellipsis)
                            }
                            _ => Ok(Token::Range)
                        }
                    }
                    _ => Ok(Token::Dot)
                }
            }
            '_' => {
                self.advance_in_same_line();
                Ok(Token::Underscore)
            }
            '?' => {
                self.advance_in_same_line();
                Ok(Token::Undefined)
            }
            '+' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('+') => {
                        self.advance_in_same_line();
                        Ok(Token::Concat)
                    }
                    _ => Ok(Token::Add)
                }
            }
            '-' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('>') => {
                        self.advance_in_same_line();
                        Ok(Token::Arrow)
                    }
                    _ => Ok(Token::Sub)
                }
            }
            '*' => {
                self.advance_in_same_line();
                Ok(Token::Mul)

            }
            '/' => {
                self.advance_in_same_line();
                Ok(Token::Div)
            }
            '^' => {
                self.advance_in_same_line();
                Ok(Token::Exp)
            }
            '=' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('=') => {
                        self.advance_in_same_line();
                        Ok(Token::Eq)
                    }
                    Some('>') => {
                        self.advance_in_same_line();
                        Ok(Token::FatArrow)
                    }
                    _ => Ok(Token::Bind)
                }
            }
            '!' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('=') => {
                        self.advance_in_same_line();
                        Ok(Token::Ne)
                    }
                    _ => Err(Error::UnsupportedOperator(self.pos.to_owned(), "!"))
                }
            }
            '>' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('=') => {
                        self.advance_in_same_line();
                        Ok(Token::Ge)
                    }
                    _ => Ok(Token::Gt)
                }
            }
            '<' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('>') => {
                        self.advance_in_same_line();
                        Ok(Token::Le)
                    }
                    _ => Ok(Token::Lt)
                }
            }
            '&' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('&') => {
                        self.advance_in_same_line();
                        Ok(Token::And)
                    }
                    _ => Ok(Token::Intersection)
                }
            }
            '|' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('|') => {
                        self.advance_in_same_line();
                        Ok(Token::Or)
                    }
                    _ => Ok(Token::Union)
                }
            }
            '@' => {
                self.advance_in_same_line();
                Ok(Token::At)
            }
            '$' => {
                self.advance_in_same_line();
                Ok(Token::Pipeline)
            }
            '~' => {
                self.advance_in_same_line();
                Ok(Token::Tilde)
            }

            other => {
                self.advance_in_same_line();
                Err(Error::UnexpectedChar(self.pos.to_owned(), other))
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_ws();

        match self.chars.peek() {
            Some('#') => {
                self.skip_comment();
                self.next()
            }
            Some('"') => {
                Some(self.lex_str_literal())
            }
            Some(&c) if c.is_ascii_digit() => {
                Some(self.lex_num_literal())
            }
            Some(&c) if c.is_alphabetic() || c == '_' => {
                Some(Ok(self.lex_id_or_keyword()))
            }
            Some(&c) => {
                Some(self.lex_others(c))
            }
            None => None,
        }
    }
}
