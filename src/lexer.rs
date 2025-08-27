use std::{collections::{HashMap, HashSet}, iter::Peekable, str::Chars};

use crate::{error::Error, token::{Pos, Span, Token, TokenKind::{self, *}}};

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    pos: Pos,

    /// Table of all Lynx keywords & corresponding `TokenKind`s
    keywords_table: HashMap<&'a str, TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            chars: src.chars().peekable(),
            pos: Pos(1, 1),

            keywords_table: HashMap::from([
                ("case"     , Case),
                ("fn"       , Fn),
                ("import"   , Import),
                ("of"       , Of),
                ("self"     , SelfLower),
                ("Self"     , SelfUpper),
                ("type"     , Type),
                ("var"      , Var),
                (":"        , Colon),
                (":"        , DoubleColon),
                ("."        , Dot),
                ("_"        , Underscore),
                ("->"       , Arrow),
                ("=>"       , FatArrow),
                ("="        , Bind),
                (":="       , Assign),
                ("@"        , At),
                ("|"        , Pipe),
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

        loop {
            match self.chars.peek() {
                Some('"') => {    // Closing quote
                    self.advance_in_same_line();
                    return Ok(Token(StrLiteral(s),
                            Span(start_pos, self.pos.to_owned())));
                }

                Some('\\') => {   // Escape sequence
                    self.advance_in_same_line();
                    let esc_start_pos = self.pos.to_owned();

                    let escaped_ch = match self.chars.peek() {
                        Some('n') => {
                            self.advance_in_same_line();
                            '\n'
                        }
                        Some('r') => {
                            self.advance_in_same_line();
                            '\r'
                        }
                        Some('t') => {
                            self.advance_in_same_line();
                            '\t'
                        }
                        Some('\\') => {
                            self.advance_in_same_line();
                            '\\'
                        }
                        Some('0') => {
                            self.advance_in_same_line();
                            '\0'
                        }
                        Some('\'') => {
                            self.advance_in_same_line();
                            '\''
                        }
                        Some('"') => {
                            self.advance_in_same_line();
                            '"'
                        }
                        // TODO: Support \u escape sequence

                        Some('\n') => {
                            self.advance_to_next_line();
                            return Err(Error::UnknownEscapeSeq(
                                    Span(esc_start_pos.to_owned(), self.pos.to_owned()),
                                    "\\\n".to_string()))
                        }
                        Some(&other) => {
                            self.advance_in_same_line();
                            return Err(Error::UnknownEscapeSeq(
                                    Span(esc_start_pos.to_owned(), self.pos.to_owned()),
                                    format!("\\{other}")))
                        }

                        None => {
                            return Err(Error::UnterminatedStr(start_pos.to_owned()))
                        }
                    };

                    s.push(escaped_ch);
                }

                // String literals may occupy multiple lines
                Some('\n') => {
                    self.advance_to_next_line();
                    s.push('\n');
                }
                Some(&c) => {
                    self.advance_in_same_line();
                    s.push(c);
                }

                None => {
                    return Err(Error::UnterminatedStr(start_pos.to_owned()));
                }
            }
        }
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
            Ok(Token(NumLiteral(num), Span(start_pos, self.pos.to_owned())))
        } else {
            Err(Error::InvalidNumFormat(Span(start_pos, self.pos.to_owned())))
        }
    }

    // Symbolic identifiers & keywords are also included
    fn lex_id_or_keyword(&mut self) -> Token {
        let start_pos = Pos(self.pos.0, self.pos.1 + 1);
        let mut name = String::new();

        while let Some(&c) = self.chars.peek() {
            if !(c.is_alphanumeric()
                    || c == '~' || c == '!'
                    || c == '@' || c == '$'
                    || c == '%' || c == '^'
                    || c == '&' || c == '*'
                    || c == '_' || c == '-'
                    || c == '+' || c == '='
                    || c == '|' || c == ':'
                    || c == '<' || c == ','
                    || c == '>' || c == '.'
                    || c == '?' || c == '/') {
                break;
            }   
            name.push(c);
            self.advance_in_same_line();
        }

        match self.keywords_table.get(name.as_str()) {
            Some(keyword_token) =>
                    Token(keyword_token.to_owned(), Span(start_pos, self.pos.to_owned())),
            None => Token(Id(name), Span(start_pos, self.pos.to_owned()))
        }
    }

    fn lex_others(&mut self, c: char) -> Result<Token, Error> {
        let start_pos = Pos(self.pos.0, self.pos.1 + 1);

        match c {
            // Lex separators & operators
            '(' => {
                self.advance_in_same_line();
                Ok(Token(Lp, Span(start_pos, self.pos.to_owned())))
            }
            ')' => {
                self.advance_in_same_line();
                Ok(Token(Rp, Span(start_pos, self.pos.to_owned())))
            }
            '[' => {
                self.advance_in_same_line();
                Ok(Token(Lb, Span(start_pos, self.pos.to_owned())))
            }
            ']' => {
                self.advance_in_same_line();
                Ok(Token(Rb, Span(start_pos, self.pos.to_owned())))
            }
            '{' => {
                self.advance_in_same_line();
                Ok(Token(Lc, Span(start_pos, self.pos.to_owned())))
            }
            '}' => {
                self.advance_in_same_line();
                Ok(Token(Rc, Span(start_pos, self.pos.to_owned())))
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
