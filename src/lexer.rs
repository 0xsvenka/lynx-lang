use std::{
    collections::{HashMap, HashSet},
    iter::Peekable,
    str::{Chars, Lines},
};

use crate::{
    error::Error,
    token::{
        Pos, Token,
        TokenKind::{self, *},
    },
};

/// Lexer that handles a single line of Lynx source code.
struct LineLexer<'a> {
    /// Peekable iterator over the line.
    chars: Peekable<Chars<'a>>,

    /// Current line number.
    line_no: usize,

    /// Current column number (before the lookahead).
    col_no: usize,

    /// Table of Lynx's alphabetic keywords & corresponding [`TokenKind`]s,
    /// used for distinguishing keywords from identifiers.
    alpha_kw_table: HashMap<&'a str, TokenKind>,

    /// Table of Lynx's symbolic keywords & corresponding [`TokenKind`]s,
    /// used for distinguishing keywords from identifiers.
    sym_kw_table: HashMap<&'a str, TokenKind>,

    /// Set of characters that are allowed in symbolic identifiers.
    sym_char_set: HashSet<char>,
}

impl<'a> LineLexer<'a> {
    /// Creates a [`LineLexer`] from a single line of Lynx source code
    /// and the line number.
    fn new(src: &'a str, line_no: usize) -> Self {
        Self {
            chars: src.chars().peekable(),
            line_no,
            col_no: 0,

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
                '\'', '<', '>', '.', '?', '/',
            ]),
        }
    }

    /// Updates the state of the lexer
    /// when advancing to the next character.
    fn advance(&mut self) {
        self.col_no += 1;
        self.chars.next();
    }

    /// Returns current position.
    fn pos(&self) -> Pos {
        Pos(self.line_no, self.col_no)
    }

    /// Skips whitespaces.
    fn skip_ws(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    /// Skips line comments,
    /// invoked when the lookahead is `--`.
    fn skip_line_comment(&mut self) {
        while let Some(_) = self.chars.peek() {
            self.advance();
        }
    }

    /// Lexes character literals,
    /// invoked when the lookahead is `'`.
    fn lex_char_lit(&mut self) -> Result<Token, Error> {
        self.advance(); // Skip opening quote
        let start_pos = self.pos();
        let mut ch_vec = Vec::new();

        loop {
            match self.chars.peek() {
                Some('\'') => {
                    // Closing quote
                    self.advance();
                    match ch_vec.len() {
                        0 => {
                            return Err(Error::EmptyCharLit(start_pos, self.pos()));
                        }
                        1 => {
                            return Ok(Token(CharLit(ch_vec[0]), start_pos, self.pos()));
                        }
                        _ => {
                            return Err(Error::MultipleCharsInCharLit(start_pos, self.pos()));
                        }
                    }
                }

                Some('\\') => {
                    // Escape sequence
                    self.advance();
                    let esc_start_pos = self.pos();

                    let escaped_ch = match self.chars.peek() {
                        Some('n') => {
                            self.advance();
                            '\n'
                        }
                        Some('r') => {
                            self.advance();
                            '\r'
                        }
                        Some('t') => {
                            self.advance();
                            '\t'
                        }
                        Some('\\') => {
                            self.advance();
                            '\\'
                        }
                        Some('0') => {
                            self.advance();
                            '\0'
                        }
                        Some('\'') => {
                            self.advance();
                            '\''
                        }
                        Some('"') => {
                            self.advance();
                            '"'
                        }
                        // TODO: Support \u escape sequences
                        Some(_) => {
                            self.advance();
                            return Err(Error::UnknownEscapeSeq(esc_start_pos, self.pos()));
                        }
                        None => {
                            return Err(Error::UnterminatedCharLit(start_pos));
                        }
                    };

                    ch_vec.push(escaped_ch);
                }

                Some(&c) => {
                    self.advance();
                    ch_vec.push(c);
                }

                None => {
                    return Err(Error::UnterminatedCharLit(start_pos));
                }
            }
        }
    }

    /// Lexes string literals,
    /// invoked when the lookahead is `"`.
    fn lex_str_lit(&mut self) -> Result<Token, Error> {
        self.advance(); // Skip opening quote
        let start_pos = self.pos();
        let mut s = String::new();

        loop {
            match self.chars.peek() {
                Some('"') => {
                    // Closing quote
                    self.advance();
                    return Ok(Token(StrLit(s), start_pos, self.pos()));
                }

                Some('\\') => {
                    // Escape sequence
                    self.advance();
                    let esc_start_pos = self.pos();

                    let escaped_ch = match self.chars.peek() {
                        Some('n') => {
                            self.advance();
                            '\n'
                        }
                        Some('r') => {
                            self.advance();
                            '\r'
                        }
                        Some('t') => {
                            self.advance();
                            '\t'
                        }
                        Some('\\') => {
                            self.advance();
                            '\\'
                        }
                        Some('0') => {
                            self.advance();
                            '\0'
                        }
                        Some('\'') => {
                            self.advance();
                            '\''
                        }
                        Some('"') => {
                            self.advance();
                            '"'
                        }
                        // TODO: Support \u escape sequences
                        Some(_) => {
                            self.advance();
                            return Err(Error::UnknownEscapeSeq(esc_start_pos, self.pos()));
                        }
                        None => {
                            return Err(Error::UnterminatedStrLit(start_pos));
                        }
                    };

                    s.push(escaped_ch);
                }

                Some(&c) => {
                    self.advance();
                    s.push(c);
                }

                None => {
                    return Err(Error::UnterminatedStrLit(start_pos));
                }
            }
        }
    }

    /// Lexes raw string literals,
    /// invoked when the lookahead is `\\`.
    fn lex_raw_string_lit(&mut self) -> Token {
        // Skip the double backslashes
        self.advance();
        let start_pos = self.pos();
        self.advance();
        let mut s = String::new();

        while let Some(&c) = self.chars.peek() {
            s.push(c);
            self.advance();
        }

        Token(StrLit(s), start_pos, self.pos())
    }

    // TODO: Support more number formats, like base prefixes and underscores
    /// Lexes number literals,
    /// invoked when the lookahead is an ascii digit.
    fn lex_num_lit(&mut self, lookahead: char) -> Result<Token, Error> {
        self.advance();
        let start_pos = self.pos();
        let mut num_str = String::new();
        num_str.push(lookahead);
        let mut is_float = false;

        while let Some(&c) = self.chars.peek() {
            match c {
                c if c.is_ascii_digit() => {
                    num_str.push(c);
                    self.advance();
                }
                '.' => {
                    if is_float {
                        break;
                    }
                    is_float = true;
                    num_str.push(c);
                    self.advance();
                }
                _ => {
                    break;
                }
            }
        }

        if is_float {
            if let Ok(num) = num_str.parse::<f64>() {
                Ok(Token(FloatLit(num), start_pos, self.pos()))
            } else {
                Err(Error::InvalidNumLitFormat(start_pos, self.pos()))
            }
        } else {
            if let Ok(num) = num_str.parse::<i64>() {
                Ok(Token(IntLit(num), start_pos, self.pos()))
            } else {
                Err(Error::InvalidNumLitFormat(start_pos, self.pos()))
            }
        }
    }

    /// Lexes alphabetic identifiers and keywords,
    /// invoked when the lookahead is alphabetic or `_`.
    fn lex_alpha(&mut self, lookahead: char) -> Token {
        self.advance();
        let start_pos = self.pos();
        let mut name = String::new();
        name.push(lookahead);

        while let Some(&c) = self.chars.peek() {
            if !(c.is_alphanumeric() || c == '_' || c == '\'' || c == '!') {
                break;
            }
            name.push(c);
            self.advance();
        }

        match self.alpha_kw_table.get(name.as_str()) {
            Some(kw_token_kind) => Token(kw_token_kind.to_owned(), start_pos, self.pos()),
            None => Token(Id(name), start_pos, self.pos()),
        }
    }

    /// Lexes symbolic identifiers and keywords,
    /// invoked when the lookahead is among:
    /// `~`, `` ` ``, `!`, `@`, `#`, `$`, `%`, `^`, `&`, `*`, `+`, `=`, `|`, `:`,
    /// `<`, `>`, `.`, `?`, `/`;
    /// in other words, [`Self::sym_char_set`] excluding `-`, `\` and `'`.
    fn lex_sym(&mut self, lookahead: char) -> Token {
        self.advance();
        let start_pos = self.pos();
        let mut name = String::new();
        name.push(lookahead);

        while let Some(&c) = self.chars.peek() {
            if !(self.sym_char_set.contains(&c)) {
                break;
            }
            name.push(c);
            self.advance();
        }

        match self.sym_kw_table.get(name.as_str()) {
            Some(kw_token_kind) => Token(kw_token_kind.to_owned(), start_pos, self.pos()),
            None => Token(Id(name), start_pos, self.pos()),
        }
    }

    /// Handles lookahead `(`.
    fn lex_lp(&mut self) -> Token {
        self.advance();
        Token(Lp, self.pos(), self.pos())
    }

    /// Handles lookahead `)`.
    fn lex_rp(&mut self) -> Token {
        self.advance();
        Token(Rp, self.pos(), self.pos())
    }

    /// Handles lookahead `[`.
    fn lex_lb(&mut self) -> Token {
        self.advance();
        Token(Lb, self.pos(), self.pos())
    }

    /// Handles lookahead `]`.
    fn lex_rb(&mut self) -> Token {
        self.advance();
        Token(Rb, self.pos(), self.pos())
    }

    /// Handles lookahead `{`.
    fn lex_lc(&mut self) -> Token {
        self.advance();
        Token(Lc, self.pos(), self.pos())
    }

    /// Handles lookahead `}`.
    fn lex_rc(&mut self) -> Token {
        self.advance();
        Token(Rc, self.pos(), self.pos())
    }

    /// Handles lookahead `,`.
    fn lex_comma(&mut self) -> Token {
        self.advance();
        Token(Comma, self.pos(), self.pos())
    }

    /// Handles lookahead `;`.
    fn lex_semicolon(&mut self) -> Token {
        self.advance();
        Token(ExprEnd, self.pos(), self.pos())
    }

    /// Handles lookahead `-`.
    fn lex_hyphen(&mut self) -> Option<Token> {
        // This is for performing the second lookahead
        let mut temp_iter = self.chars.clone();
        temp_iter.next();
        match temp_iter.peek() {
            // `--`: line comment
            Some('-') => {
                self.skip_line_comment();
                None
            }
            // Otherwise, the beginning of a symbolic identifier
            _ => Some(self.lex_sym('-')),
        }
    }

    /// Handles lookahead `\`.
    fn lex_backslash(&mut self) -> Token {
        // This is for performing the second lookahead
        let mut temp_iter = self.chars.clone();
        temp_iter.next();
        match temp_iter.peek() {
            // `\\`: raw string literal
            Some('\\') => self.lex_raw_string_lit(),
            // Otherwise, the beginning of a symbolic identifier
            _ => self.lex_sym('\\'),
        }
    }
}

/// [`LineLexer`] serves as an iterator emitting [`Result<Token, Error>`]s.
impl<'a> Iterator for LineLexer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_ws();

        match self.chars.peek()? {
            '(' => Some(Ok(self.lex_lp())),
            ')' => Some(Ok(self.lex_rp())),
            '[' => Some(Ok(self.lex_lb())),
            ']' => Some(Ok(self.lex_rb())),
            '{' => Some(Ok(self.lex_lc())),
            '}' => Some(Ok(self.lex_rc())),
            ',' => Some(Ok(self.lex_comma())),
            ';' => Some(Ok(self.lex_semicolon())),
            '-' => self.lex_hyphen().map(|token| Ok(token)),
            '\\' => Some(Ok(self.lex_backslash())),
            '\'' => Some(self.lex_char_lit()),
            '"' => Some(self.lex_str_lit()),
            &c if c.is_ascii_digit() => Some(self.lex_num_lit(c)),
            &c if c.is_alphabetic() || c == '_' => Some(Ok(self.lex_alpha(c))),
            &c if self.sym_char_set.contains(&c) => Some(Ok(self.lex_sym(c))),

            // The lookahead cannot be lexed
            _ => {
                self.advance();
                Some(Err(Error::UnexpectedChar(self.pos())))
            }
        }
    }
}

pub struct Lexer<'a> {
    lines: Lines<'a>,
    current_line_lexer: LineLexer<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut lines = src.lines();
        let current_line_lexer = lines.next();
        todo!()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
