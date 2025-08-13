use std::{collections::HashMap, iter::Peekable, str::Chars};

use crate::{error::Error, token::{Pos, Span, Token, TokenKind::{self, *}}};

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
                ("break"   , Break),
                ("continue", Continue),
                ("do"      , Do),
                ("else"    , Else),
                ("fn"      , Fn),
                ("for"     , For),
                ("if"      , If),
                ("import"  , Import),
                ("in"      , In),
                ("match"   , Match),
                ("mod"     , Mod),
                ("not"     , Not),
                ("return"  , Return),
                ("self"    , Self_),
                ("then"    , Then),
                ("type"    , Type),
                ("var"     , Var),
                ("while"   , While),
                ("with"    , With),
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

    fn lex_id_or_keyword_or_underscore(&mut self) -> Token {
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

        if name.as_str() == "_" {
            return Token(Underscore, Span(start_pos, self.pos.to_owned()));
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
            ':' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('=') => {
                        self.advance_in_same_line();
                        Ok(Token(Assign, Span(start_pos, self.pos.to_owned())))
                    }
                    Some(':') => {
                        self.advance_in_same_line();
                        Ok(Token(DoubleColon, Span(start_pos, self.pos.to_owned())))
                    }
                    _ => Ok(Token(Colon, Span(start_pos, self.pos.to_owned())))
                }
            }
            ',' => {
                self.advance_in_same_line();
                Ok(Token(Comma, Span(start_pos, self.pos.to_owned())))
            }
            ';' => {
                self.advance_in_same_line();
                Ok(Token(ExprEnd, Span(start_pos, self.pos.to_owned())))
            }
            '\n' => {
                self.advance_to_next_line();
                Ok(Token(ExprEnd, Span(start_pos, self.pos.to_owned())))
            }
            '\\' => {
                self.advance_in_same_line();
                Ok(Token(ExprContinue, Span(start_pos, self.pos.to_owned())))
            }
            '.' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('.') => {
                        self.advance_in_same_line();
                        match self.chars.peek() {
                            Some('.') => {
                                self.advance_in_same_line();
                                Ok(Token(Ellipsis, Span(start_pos, self.pos.to_owned())))
                            }
                            _ => Ok(Token(Range, Span(start_pos, self.pos.to_owned())))
                        }
                    }
                    _ => Ok(Token(Dot, Span(start_pos, self.pos.to_owned())))
                }
            }
            '?' => {
                self.advance_in_same_line();
                Ok(Token(Undefined, Span(start_pos, self.pos.to_owned())))
            }
            '+' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('+') => {
                        self.advance_in_same_line();
                        Ok(Token(Concat, Span(start_pos, self.pos.to_owned())))
                    }
                    _ => Ok(Token(Add, Span(start_pos, self.pos.to_owned())))
                }
            }
            '-' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('>') => {
                        self.advance_in_same_line();
                        Ok(Token(Arrow, Span(start_pos, self.pos.to_owned())))
                    }
                    _ => Ok(Token(Sub, Span(start_pos, self.pos.to_owned())))
                }
            }
            '*' => {
                self.advance_in_same_line();
                Ok(Token(Mul, Span(start_pos, self.pos.to_owned())))

            }
            '/' => {
                self.advance_in_same_line();
                Ok(Token(Div, Span(start_pos, self.pos.to_owned())))
            }
            '^' => {
                self.advance_in_same_line();
                Ok(Token(Exp, Span(start_pos, self.pos.to_owned())))
            }
            '=' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('=') => {
                        self.advance_in_same_line();
                        Ok(Token(Eq, Span(start_pos, self.pos.to_owned())))
                    }
                    Some('>') => {
                        self.advance_in_same_line();
                        Ok(Token(FatArrow, Span(start_pos, self.pos.to_owned())))
                    }
                    _ => Ok(Token(Bind, Span(start_pos, self.pos.to_owned())))
                }
            }
            '!' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('=') => {
                        self.advance_in_same_line();
                        Ok(Token(Ne, Span(start_pos, self.pos.to_owned())))
                    }
                    _ => Err(Error::UnsupportedOperator(
                            Span(start_pos, self.pos.to_owned()), "!".to_string()))
                }
            }
            '>' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('=') => {
                        self.advance_in_same_line();
                        Ok(Token(Ge, Span(start_pos, self.pos.to_owned())))
                    }
                    _ => Ok(Token(Gt, Span(start_pos, self.pos.to_owned())))
                }
            }
            '<' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('>') => {
                        self.advance_in_same_line();
                        Ok(Token(Le, Span(start_pos, self.pos.to_owned())))
                    }
                    _ => Ok(Token(Lt, Span(start_pos, self.pos.to_owned())))
                }
            }
            '&' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('&') => {
                        self.advance_in_same_line();
                        Ok(Token(And, Span(start_pos, self.pos.to_owned())))
                    }
                    _ => Ok(Token(Intersection, Span(start_pos, self.pos.to_owned())))
                }
            }
            '|' => {
                self.advance_in_same_line();
                match self.chars.peek() {
                    Some('|') => {
                        self.advance_in_same_line();
                        Ok(Token(Or, Span(start_pos, self.pos.to_owned())))
                    }
                    _ => Ok(Token(Union, Span(start_pos, self.pos.to_owned())))
                }
            }
            '@' => {
                self.advance_in_same_line();
                Ok(Token(At, Span(start_pos, self.pos.to_owned())))
            }
            '$' => {
                self.advance_in_same_line();
                Ok(Token(Pipeline, Span(start_pos, self.pos.to_owned())))
            }
            '~' => {
                self.advance_in_same_line();
                Ok(Token(Tilde, Span(start_pos, self.pos.to_owned())))
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
                Some(Ok(self.lex_id_or_keyword_or_underscore()))
            }
            Some(&c) => {
                Some(self.lex_others(c))
            }
            None => None,
        }
    }
}
