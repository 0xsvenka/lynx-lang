use std::{collections::HashMap, iter::Peekable, str::Chars};

use crate::{error::Error, token::{Pos, Span, Token, TokenKind::{self, *}}};

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    pos: Pos,

    /// Table of Lynx's alphabetic keywords & corresponding `TokenKind`s
    alpha_kw_table: HashMap<&'a str, TokenKind>,
    /// Table of Lynx's symbolic keywords & corresponding `TokenKind`s.
    /// This is used for distinguishing keywords from identifiers,
    /// and since `(`, `)`, `[`, `]`, `{`, `}`, `,`, `;` and `\` are
    /// not allowed in identifiers, the keywords containing them are
    /// left out in this table and lexed differently.
    sym_kw_table: HashMap<&'a str, TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            chars: src.chars().peekable(),
            pos: Pos(1, 1),

            alpha_kw_table: HashMap::from([
                ("case"     , Case),
                ("import"   , Import),
                ("of"       , Of),
            ]),
            sym_kw_table: HashMap::from([
                (":"        , Colon),
                ("::"       , DoubleColon),
                ("."        , Dot),
                ("_"        , Underscore),
                ("->"       , Arrow),
                ("=>"       , FatArrow),
                ("="        , Bind),
                ("@"        , At),
                ("|"        , Pipe),
                ("%"        , Percent),
                ("~"        , Tilde),
                ("%~"       , PercentTilde),
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

    /// This function does not skip `\n`, since it
    /// should be lexed as [crate::token::TokenKind::ExprEnd].
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
                    return Ok(Token(StrLit(s),
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
                        // TODO: Support \u escape sequences

                        Some('\n') => {
                            self.advance_to_next_line();
                            return Err(Error::UnknownEscapeSeq(
                                    Span(esc_start_pos.to_owned(), self.pos.to_owned()),
                                    "\\\n".to_string()));
                        }
                        Some(&other) => {
                            self.advance_in_same_line();
                            return Err(Error::UnknownEscapeSeq(
                                    Span(esc_start_pos.to_owned(), self.pos.to_owned()),
                                    format!("\\{other}")));
                        }

                        None => {
                            return Err(Error::UnterminatedStr(start_pos.to_owned()));
                        }
                    };

                    s.push(escaped_ch);
                }

                // String literals may occupy multiple lines,
                // and the line breaks are preserved.
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

    // TODO: Support more number formats, like base prefixes and underscores
    fn lex_num_literal(&mut self) -> Result<Token, Error> {
        let start_pos = Pos(self.pos.0, self.pos.1 + 1);
        let mut num_str = String::new();
        let mut is_float = false;
        
        while let Some(&c) = self.chars.peek() {
            match c {
                c if c.is_ascii_digit() => {
                    num_str.push(c);
                    self.advance_in_same_line();
                }
                '.' => {
                    if is_float {
                        break;
                    }
                    is_float = true;
                    num_str.push(c);
                    self.advance_in_same_line();
                }
                _ => {
                    break;
                }
            }
        }
        
        if is_float {
            if let Ok(num) = num_str.parse::<f64>() {
                Ok(Token(FloatLit(num),
                    Span(start_pos, self.pos.to_owned())))
            } else {
                Err(Error::InvalidNumFormat(
                    Span(start_pos, self.pos.to_owned())))
            }
        } else {
            if let Ok(num) = num_str.parse::<i64>() {
                Ok(Token(IntLit(num),
                    Span(start_pos, self.pos.to_owned())))
            } else {
                Err(Error::InvalidNumFormat(
                    Span(start_pos, self.pos.to_owned())))
            }
        }
    }

    fn lex_alpha(&mut self) -> Token {
        let start_pos = Pos(self.pos.0, self.pos.1 + 1);
        let mut name = String::new();

        while let Some(&c) = self.chars.peek() {
            if !(c.is_alphanumeric()
                    || c == '\'' || c == '!' || c == '_') {
                break;
            }   
            name.push(c);
            self.advance_in_same_line();
        }

        match self.alpha_kw_table.get(name.as_str()) {
            Some(keyword_token) =>
                    Token(keyword_token.to_owned(),
                        Span(start_pos, self.pos.to_owned())),
            None => Token(Id(name),
                        Span(start_pos, self.pos.to_owned())),
        }
    }

    fn lex_sym(&mut self) -> Token {
        let start_pos = Pos(self.pos.0, self.pos.1 + 1);
        let mut name = String::new();

        while let Some(&c) = self.chars.peek() {
            if !(c == '~' || c == '`' || c == '!' ||
                 c == '@' || c == '$' || c == '%' ||
                 c == '^' || c == '&' || c == '*' ||
                 c == '-' || c == '+' || c == '=' ||
                 c == '|' || c == ':' || c == '<' ||
                 c == '>' || c == '.' || c == '?' ||
                 c == '/' || c == '\''|| c =='_')
            {
                break;
            }   
            name.push(c);
            self.advance_in_same_line();
        }

        match self.sym_kw_table.get(name.as_str()) {
            Some(keyword_token) =>
                    Token(keyword_token.to_owned(),
                        Span(start_pos, self.pos.to_owned())),
            None => Token(Id(name),
                        Span(start_pos, self.pos.to_owned())),
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
            // TODO: Char literals
            Some('"') => {
                Some(self.lex_str_literal())
            }
            Some(&c) if c.is_ascii_digit() => {
                Some(self.lex_num_literal())
            }
            Some(&c) if c.is_alphabetic() || c == '_' => {
                Some(Ok(self.lex_alpha()))
            }
            Some(&c)
                 if c == '~' || c == '`' || c == '!' ||
                    c == '@' || c == '$' || c == '%' ||
                    c == '^' || c == '&' || c == '*' ||
                    c == '-' || c == '+' || c == '=' ||
                    c == '|' || c == ':' || c == '<' ||
                    c == '>' || c == '.' || c == '?' ||
                    c == '/'
                => {
                Some(Ok(self.lex_sym()))
            }
            Some(&c) => {
                Some(self.lex_others(c))
            }
            None => None,
        }
    }
}
