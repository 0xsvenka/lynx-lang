use std::{collections::HashMap, iter::Peekable, str::Chars};

use crate::{error::Error, token::{Pos, Span, Token, TokenKind}};

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    pos: Pos,

    keywords_table: HashMap<&'a str, TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            chars: src.chars().peekable(),
            pos: Pos(1, 1),

            keywords_table: HashMap::from([
                ("break"   , TokenKind::Break),
                ("continue", TokenKind::Continue),
                ("do"      , TokenKind::Do),
                ("else"    , TokenKind::Else),
                ("fn"      , TokenKind::Fn),
                ("for"     , TokenKind::For),
                ("if"      , TokenKind::If),
                ("import"  , TokenKind::Import),
                ("in"      , TokenKind::In),
                ("match"   , TokenKind::Match),
                ("mod"     , TokenKind::Mod),
                ("not"     , TokenKind::Not),
                ("return"  , TokenKind::Return),
                ("self"    , TokenKind::Self_),
                ("then"    , TokenKind::Then),
                ("type"    , TokenKind::Type),
                ("var"     , TokenKind::Var),
                ("while"   , TokenKind::While),
                ("with"    , TokenKind::With),
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
        self.advance_in_same_line();    // Skip opening quote
        let start_pos = self.pos.to_owned();
        let mut s = String::new();

        while let Some(&c) = self.chars.peek() {
            match c {
                '"' => {    // Closing quote
                    self.advance_in_same_line();
                    return Ok(Token(TokenKind::StrLiteral(s),
                            Span(start_pos, self.pos.to_owned())));
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
        let start_pos = Pos(self.pos.0, self.pos.1 + 1);
        let mut num_str = String::new();

        while let Some(&c) = self.chars.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            num_str.push(c);
            self.advance_in_same_line();
        }

        if let Ok(num) = num_str.parse() {
            Ok(Token(TokenKind::NumLiteral(num), Span(start_pos, self.pos.to_owned())))
        } else {
            Err(Error::InvalidNumFormat(Span(start_pos, self.pos.to_owned())))
        }
    }

    fn lex_id_or_keyword(&mut self) -> Token {
        let start_pos = Pos(self.pos.0, self.pos.1 + 1);
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
            Some(keyword_token) =>
                    Token(keyword_token.to_owned(), Span(start_pos, self.pos.to_owned())),
            None => Token(TokenKind::Id(name), Span(start_pos, self.pos.to_owned()))
        }
    }

    fn lex_others(&mut self, c: char) -> Result<Token, Error> {
        let start_pos = Pos(self.pos.0, self.pos.1 + 1);

        match c {
            // Lex separators & operators
            '(' => {
                self.advance_in_same_line();
                Ok(Token(TokenKind::Lp, Span(start_pos, self.pos.to_owned())))
            }
            ')' => {
                self.advance_in_same_line();
                Ok(Token(TokenKind::Rp, Span(start_pos, self.pos.to_owned())))
            }
            '[' => {
                self.advance_in_same_line();
                Ok(Token(TokenKind::Lb, Span(start_pos, self.pos.to_owned())))
            }
            ']' => {
                self.advance_in_same_line();
                Ok(Token(TokenKind::Rb, Span(start_pos, self.pos.to_owned())))
            }
            '{' => {
                self.advance_in_same_line();
                Ok(Token(TokenKind::Lc, Span(start_pos, self.pos.to_owned())))
            }
            '}' => {
                self.advance_in_same_line();
                Ok(Token(TokenKind::Rc, Span(start_pos, self.pos.to_owned())))
            }
            ':' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('=') => {
                        self.advance_in_same_line();
                        Ok(Token(TokenKind::Assign, Span(start_pos, self.pos.to_owned())))
                    }
                    Some(':') => {
                        self.advance_in_same_line();
                        Ok(Token(TokenKind::DoubleColon, Span(start_pos, self.pos.to_owned())))
                    }
                    _ => Ok(Token(TokenKind::Colon, Span(start_pos, self.pos.to_owned())))
                }
            }
            ',' => {
                self.advance_in_same_line();
                Ok(Token(TokenKind::Comma, Span(start_pos, self.pos.to_owned())))
            }
            ';' => {
                self.advance_in_same_line();
                Ok(Token(TokenKind::ExprEnd, Span(start_pos, self.pos.to_owned())))
            }
            '\n' => {
                self.advance_to_next_line();
                Ok(Token(TokenKind::ExprEnd, Span(start_pos, self.pos.to_owned())))
            }
            '\\' => {
                self.advance_in_same_line();
                Ok(Token(TokenKind::ExprContinue, Span(start_pos, self.pos.to_owned())))
            }
            '.' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('.') => {
                        self.advance_in_same_line();
                        match self.chars.peek() {
                            Some('.') => {
                                self.advance_in_same_line();
                                Ok(Token(TokenKind::Ellipsis, Span(start_pos, self.pos.to_owned())))
                            }
                            _ => Ok(Token(TokenKind::Range, Span(start_pos, self.pos.to_owned())))
                        }
                    }
                    _ => Ok(Token(TokenKind::Dot, Span(start_pos, self.pos.to_owned())))
                }
            }
            '_' => {
                self.advance_in_same_line();
                Ok(Token(TokenKind::Underscore, Span(start_pos, self.pos.to_owned())))
            }
            '?' => {
                self.advance_in_same_line();
                Ok(Token(TokenKind::Undefined, Span(start_pos, self.pos.to_owned())))
            }
            '+' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('+') => {
                        self.advance_in_same_line();
                        Ok(Token(TokenKind::Concat, Span(start_pos, self.pos.to_owned())))
                    }
                    _ => Ok(Token(TokenKind::Add, Span(start_pos, self.pos.to_owned())))
                }
            }
            '-' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('>') => {
                        self.advance_in_same_line();
                        Ok(Token(TokenKind::Arrow, Span(start_pos, self.pos.to_owned())))
                    }
                    _ => Ok(Token(TokenKind::Sub, Span(start_pos, self.pos.to_owned())))
                }
            }
            '*' => {
                self.advance_in_same_line();
                Ok(Token(TokenKind::Mul, Span(start_pos, self.pos.to_owned())))

            }
            '/' => {
                self.advance_in_same_line();
                Ok(Token(TokenKind::Div, Span(start_pos, self.pos.to_owned())))
            }
            '^' => {
                self.advance_in_same_line();
                Ok(Token(TokenKind::Exp, Span(start_pos, self.pos.to_owned())))
            }
            '=' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('=') => {
                        self.advance_in_same_line();
                        Ok(Token(TokenKind::Eq, Span(start_pos, self.pos.to_owned())))
                    }
                    Some('>') => {
                        self.advance_in_same_line();
                        Ok(Token(TokenKind::FatArrow, Span(start_pos, self.pos.to_owned())))
                    }
                    _ => Ok(Token(TokenKind::Bind, Span(start_pos, self.pos.to_owned())))
                }
            }
            '!' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('=') => {
                        self.advance_in_same_line();
                        Ok(Token(TokenKind::Ne, Span(start_pos, self.pos.to_owned())))
                    }
                    _ => Err(Error::UnsupportedOperator(
                            Span(start_pos, self.pos.to_owned()), "!"))
                }
            }
            '>' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('=') => {
                        self.advance_in_same_line();
                        Ok(Token(TokenKind::Ge, Span(start_pos, self.pos.to_owned())))
                    }
                    _ => Ok(Token(TokenKind::Gt, Span(start_pos, self.pos.to_owned())))
                }
            }
            '<' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('>') => {
                        self.advance_in_same_line();
                        Ok(Token(TokenKind::Le, Span(start_pos, self.pos.to_owned())))
                    }
                    _ => Ok(Token(TokenKind::Lt, Span(start_pos, self.pos.to_owned())))
                }
            }
            '&' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('&') => {
                        self.advance_in_same_line();
                        Ok(Token(TokenKind::And, Span(start_pos, self.pos.to_owned())))
                    }
                    _ => Ok(Token(TokenKind::Intersection, Span(start_pos, self.pos.to_owned())))
                }
            }
            '|' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('|') => {
                        self.advance_in_same_line();
                        Ok(Token(TokenKind::Or, Span(start_pos, self.pos.to_owned())))
                    }
                    _ => Ok(Token(TokenKind::Union, Span(start_pos, self.pos.to_owned())))
                }
            }
            '@' => {
                self.advance_in_same_line();
                Ok(Token(TokenKind::At, Span(start_pos, self.pos.to_owned())))
            }
            '$' => {
                self.advance_in_same_line();
                Ok(Token(TokenKind::Pipeline, Span(start_pos, self.pos.to_owned())))
            }
            '~' => {
                self.advance_in_same_line();
                Ok(Token(TokenKind::Tilde, Span(start_pos, self.pos.to_owned())))
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
