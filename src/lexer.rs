use std::{collections::HashMap, collections::HashSet, iter::Peekable, str::Chars};

use crate::{
    error::Error,
    token::{
        Pos, Span, Token,
        TokenKind::{self, *},
    },
};

/// Lexer for Lynx code.
pub struct Lexer<'a> {
    /// A peekable iterator over the source text.
    chars: Peekable<Chars<'a>>,

    /// Current position in the source text.
    pos: Pos,

    /// Table of Lynx's alphabetic keywords & corresponding [`TokenKind`]s,
    /// used for distinguishing keywords from identifiers.
    alpha_kw_table: HashMap<&'a str, TokenKind>,

    /// Table of Lynx's symbolic keywords & corresponding [`TokenKind`]s,
    /// used for distinguishing keywords from identifiers.
    ///
    /// Since `(`, `)`, `[`, `]`, `{`, `}`, `,`, and `;` are
    /// not allowed in identifiers, the corresponding keywords are
    /// left out from this table.
    sym_kw_table: HashMap<&'a str, TokenKind>,

    /// Set of characters that are allowed as the _beginning_ of a
    /// symbolic identifier.
    sym_char_set: HashSet<char>,
}

impl<'a> Lexer<'a> {
    /// Creates a [`Lexer`] from a [`&str`](str).
    pub fn new(src: &'a str) -> Self {
        Self {
            chars: src.chars().peekable(),
            pos: Pos(1, 0),

            alpha_kw_table: HashMap::from([("ctor", Ctor), ("import", Import), ("_", Underscore)]),
            sym_kw_table: HashMap::from([
                (":", Colon),
                ("::", DoubleColon),
                (".", Dot),
                ("->", Arrow),
                ("=>", FatArrow),
                ("=", Bind),
                ("@", At),
                ("|", Pipe),
                ("#", Hash),
                ("%", Percent),
                ("~", Tilde),
                ("%~", PercentTilde),
            ]),
            sym_char_set: HashSet::from([
                '~', '`', '!', '@', '#', '$', '%', '^', '&', '*', '-', '+', '=', '|', '\\', ':',
                '<', '>', '.', '?', '/',
            ]),
        }
    }

    /// Updates the state of the lexer
    /// when we are advancing in the same line.
    #[inline]
    fn advance_in_same_line(&mut self) {
        self.pos.1 += 1;
        self.chars.next();
    }

    /// Updates the state of the lexer
    /// when we are advancing to the _line break_.
    ///
    /// After calling this function,
    /// `self.pos` becomes [`Pos(/*next_line_number*/, 0)`](crate::token::Pos).
    #[inline]
    fn advance_to_next_line(&mut self) {
        self.pos.0 += 1;
        self.pos.1 = 0;
        self.chars.next();
    }

    /// Skips whitespaces.
    ///
    /// Note that `\n` is not skipped here and is handled in
    /// [`Lexer::lex_eol`] instead.
    fn skip_ws(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if !c.is_whitespace() || c == '\n' {
                break;
            }
            self.advance_in_same_line();
        }
    }

    /// Skips line comments starting with `--`.
    ///
    /// Note that the `\n` at the end of the line is not
    /// skipped here and is handled in [`Lexer""lex_eol`] instead.
    fn skip_line_comment(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c == '\n' {
                break;
            } else {
                self.advance_in_same_line();
            }
        }
    }

    /// Lexes character literals.
    fn lex_char_lit(&mut self) -> Result<Token, Error> {
        self.advance_in_same_line(); // Skip opening quote
        let start_pos = self.pos;
        let mut ch_vec = Vec::new();

        loop {
            match self.chars.peek() {
                Some('\'') => {
                    // Closing quote
                    self.advance_in_same_line();
                    match ch_vec.len() {
                        0 => {
                            return Err(Error::EmptyCharLit(Span(start_pos, self.pos)));
                        }
                        1 => {
                            return Ok(Token(CharLit(ch_vec[0]), Span(start_pos, self.pos)));
                        }
                        _ => {
                            return Err(Error::MultipleCharsInCharLit(Span(start_pos, self.pos)));
                        }
                    }
                }

                Some('\\') => {
                    // Escape sequence
                    self.advance_in_same_line();
                    let esc_start_pos = self.pos;

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
                        Some(&c) => {
                            if c == '\n' {
                                self.advance_to_next_line();
                            } else {
                                self.advance_in_same_line();
                            }
                            return Err(Error::UnknownEscapeSeq(Span(esc_start_pos, self.pos)));
                        }
                        None => {
                            return Err(Error::UnterminatedCharLit(start_pos));
                        }
                    };

                    ch_vec.push(escaped_ch);
                }

                Some(&c) => {
                    // A character literal may contain a line break
                    if c == '\n' {
                        self.advance_to_next_line();
                    } else {
                        self.advance_in_same_line();
                    }
                    ch_vec.push(c);
                }

                None => {
                    return Err(Error::UnterminatedCharLit(start_pos));
                }
            }
        }
    }

    /// Lexes string literals.
    fn lex_str_lit(&mut self) -> Result<Token, Error> {
        self.advance_in_same_line(); // Skip opening quote
        let start_pos = self.pos;
        let mut s = String::new();

        loop {
            match self.chars.peek() {
                Some('"') => {
                    // Closing quote
                    self.advance_in_same_line();
                    return Ok(Token(StrLit(s), Span(start_pos, self.pos)));
                }

                Some('\\') => {
                    // Escape sequence
                    self.advance_in_same_line();
                    let esc_start_pos = self.pos;

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
                        Some(&c) => {
                            if c == '\n' {
                                self.advance_to_next_line();
                            } else {
                                self.advance_in_same_line();
                            }
                            return Err(Error::UnknownEscapeSeq(Span(esc_start_pos, self.pos)));
                        }
                        None => {
                            return Err(Error::UnterminatedStrLit(start_pos));
                        }
                    };

                    s.push(escaped_ch);
                }

                Some(&c) => {
                    // String literals may occupy multiple lines,
                    // and the line breaks are preserved.
                    if c == '\n' {
                        self.advance_to_next_line();
                    } else {
                        self.advance_in_same_line();
                    }
                    s.push(c);
                }

                None => {
                    return Err(Error::UnterminatedStrLit(start_pos));
                }
            }
        }
    }

    // TODO: Support more number formats, like base prefixes and underscores
    /// Lexes number literals.
    fn lex_num_lit(&mut self, lookahead: char) -> Result<Token, Error> {
        self.advance_in_same_line();
        let start_pos = self.pos;
        let mut num_str = String::new();
        num_str.push(lookahead);
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
                Ok(Token(FloatLit(num), Span(start_pos, self.pos)))
            } else {
                Err(Error::InvalidNumLitFormat(Span(start_pos, self.pos)))
            }
        } else {
            if let Ok(num) = num_str.parse::<i64>() {
                Ok(Token(IntLit(num), Span(start_pos, self.pos)))
            } else {
                Err(Error::InvalidNumLitFormat(Span(start_pos, self.pos)))
            }
        }
    }

    /// Lexes alphabetic identifiers and keywords.
    fn lex_alpha(&mut self, lookahead: char) -> Token {
        self.advance_in_same_line();
        let start_pos = self.pos;
        let mut name = String::new();
        name.push(lookahead);

        while let Some(&c) = self.chars.peek() {
            if !(c.is_alphanumeric() || c == '_' || c == '\'' || c == '!') {
                break;
            }
            name.push(c);
            self.advance_in_same_line();
        }

        match self.alpha_kw_table.get(name.as_str()) {
            Some(kw_token_kind) => Token(kw_token_kind.to_owned(), Span(start_pos, self.pos)),
            None => Token(Id(name), Span(start_pos, self.pos)),
        }
    }

    /// Lexes symbolic identifiers and keywords.
    ///
    /// Since characters `(`, `)`, `[`, `]`, `{`, `}`, `,`, and `;`
    /// are not allowed in identifiers, the corresponding keywords
    /// are not handled by this function but lexed separately instead.
    fn lex_sym(&mut self, lookahead: char) -> Token {
        self.advance_in_same_line();
        let start_pos = self.pos;
        let mut name = String::new();
        name.push(lookahead);

        while let Some(&c) = self.chars.peek() {
            if !(self.sym_char_set.contains(&c) || c == '_' || c == '\'') {
                break;
            }
            name.push(c);
            self.advance_in_same_line();
        }

        match self.sym_kw_table.get(name.as_str()) {
            Some(kw_token_kind) => Token(kw_token_kind.to_owned(), Span(start_pos, self.pos)),
            None => Token(Id(name), Span(start_pos, self.pos)),
        }
    }

    /// Handles lookahead `(`.
    fn lex_lp(&mut self) -> Token {
        self.advance_in_same_line();
        Token(Lp, Span(self.pos, self.pos))
    }

    /// Handles lookahead `)`.
    fn lex_rp(&mut self) -> Token {
        self.advance_in_same_line();
        Token(Rp, Span(self.pos, self.pos))
    }

    /// Handles lookahead `[`.
    fn lex_lb(&mut self) -> Token {
        self.advance_in_same_line();
        Token(Lb, Span(self.pos, self.pos))
    }

    /// Handles lookahead `]`.
    fn lex_rb(&mut self) -> Token {
        self.advance_in_same_line();
        Token(Rb, Span(self.pos, self.pos))
    }

    /// Handles lookahead `{`.
    fn lex_lc(&mut self) -> Token {
        self.advance_in_same_line();
        Token(Lc, Span(self.pos, self.pos))
    }

    /// Handles lookahead `}`.
    fn lex_rc(&mut self) -> Token {
        self.advance_in_same_line();
        Token(Rc, Span(self.pos, self.pos))
    }

    /// Handles lookahead `,`.
    fn lex_comma(&mut self) -> Token {
        self.advance_in_same_line();
        Token(Comma, Span(self.pos, self.pos))
    }

    /// Handles lookahead `;`.
    fn lex_semicolon(&mut self) -> Token {
        self.advance_in_same_line();
        Token(ExprEnd, Span(self.pos, self.pos))
    }

    /// Handles lookahead `\n`.
    fn lex_eol(&mut self) -> Token {
        todo!()
    }
}

/// The [`Lexer`] serves as an iterator generating [`Result<Token, Error>`]s.
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_ws();

        match self.chars.peek()? {
            '\'' => Some(self.lex_char_lit()),
            '"' => Some(self.lex_str_lit()),
            &c if c.is_ascii_digit() => Some(self.lex_num_lit(c)),
            &c if c.is_alphabetic() || c == '_' => Some(Ok(self.lex_alpha(c))),
            &c if self.sym_char_set.contains(&c) => Some(Ok(self.lex_sym(c))),
            '(' => Some(Ok(self.lex_lp())),
            ')' => Some(Ok(self.lex_rp())),
            '[' => Some(Ok(self.lex_lb())),
            ']' => Some(Ok(self.lex_rb())),
            '{' => Some(Ok(self.lex_lc())),
            '}' => Some(Ok(self.lex_rc())),
            ',' => Some(Ok(self.lex_comma())),
            ';' => Some(Ok(self.lex_semicolon())),
            '\n' => Some(Ok(self.lex_eol())),

            // The lookahead cannot be lexed
            _ => {
                self.advance_in_same_line();
                Some(Err(Error::UnexpectedChar(self.pos)))
            }
        }
    }
}
