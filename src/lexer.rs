use std::{collections::HashMap, iter::Peekable, str::Chars};

use crate::{
    error::Error,
    token::{
        Pos, Span, Token,
        TokenKind::{self, *},
    },
};

/// Lexer for Lynx source code. It recognizes [`Token`]s (or [`Error`]s)
/// from a string.
pub struct Lexer<'a> {
    /// A peekable iterator over the source code.
    chars: Peekable<Chars<'a>>,

    /// Current position in the source file.
    pos: Pos,

    /// Table of Lynx's alphabetic keywords & corresponding [`TokenKind`]s.
    alpha_kw_table: HashMap<&'a str, TokenKind>,
    /// Table of Lynx's symbolic keywords & corresponding [`TokenKind`]s.
    /// This is used for distinguishing keywords from identifiers,
    /// and since `(`, `)`, `[`, `]`, `{`, `}`, `,`, `;` and `\` are
    /// not allowed in identifiers, the keywords containing them are
    /// left out from this table and lexed separately.
    sym_kw_table: HashMap<&'a str, TokenKind>,
}

impl<'a> Lexer<'a> {
    /// Creates a [`Lexer`] from a [`&str`](str).
    pub fn new(src: &'a str) -> Self {
        Self {
            chars: src.chars().peekable(),
            pos: Pos(1, 0),

            alpha_kw_table: HashMap::from([
                ("case", Case),
                ("import", Import),
                ("infix", Infix),
                ("infixl", Infixl),
                ("infixr", Infixr),
                ("of", Of),
                ("_", Underscore),
            ]),
            sym_kw_table: HashMap::from([
                (":", Colon),
                ("::", DoubleColon),
                (".", Dot),
                ("->", Arrow),
                ("=>", FatArrow),
                ("=", Bind),
                ("@", At),
                ("|", Pipe),
                ("%", Percent),
                ("~", Tilde),
                ("%~", PercentTilde),
            ]),
        }
    }

    /// Updates the inner state of the lexer,
    /// when we are advancing in the same line.
    #[inline]
    fn advance_in_same_line(&mut self) {
        self.pos.1 += 1;
        self.chars.next();
    }

    /// Updates the inner state of the lexer,
    /// when we are advancing to the *line break*.
    /// After calling this function, `self.pos`
    /// becomes `Pos(/*next_line_number*/, 0)`.
    #[inline]
    fn advance_to_next_line(&mut self) {
        self.pos.0 += 1;
        // We haven't come to the first character of the next line yet
        self.pos.1 = 0;
        self.chars.next();
    }

    /// Skips whitespaces. Note that this function does
    /// not skip `\n`, since it should be lexed as [`ExprEnd`].
    fn skip_ws(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if !c.is_whitespace() || c == '\n' {
                break;
            }
            self.advance_in_same_line();
        }
    }

    /// Skips line comment starting with `#`. Note that
    /// this function does not skip `\n` at the end of the
    /// line, since it should be lexed as [`ExprEnd`].
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
    fn lex_num_lit(&mut self) -> Result<Token, Error> {
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
    fn lex_alpha(&mut self) -> Token {
        let start_pos = Pos(self.pos.0, self.pos.1 + 1);
        let mut name = String::new();

        while let Some(&c) = self.chars.peek() {
            if !(c.is_alphanumeric() || c == '\'' || c == '!' || c == '_') {
                break;
            }
            name.push(c);
            self.advance_in_same_line();
        }

        match self.alpha_kw_table.get(name.as_str()) {
            Some(keyword_token) => Token(keyword_token.to_owned(), Span(start_pos, self.pos)),
            None => Token(Id(name), Span(start_pos, self.pos)),
        }
    }

    /// Lexes symbolic identifiers and keywords.
    /// Since `(`, `)`, `[`, `]`, `{`, `}`, `,`, `;` and `\` are
    /// not allowed in identifiers, the keywords containing them are
    /// not handled in this function. Instead, they are lexed separately.
    fn lex_sym(&mut self) -> Token {
        let start_pos = Pos(self.pos.0, self.pos.1 + 1);
        let mut name = String::new();

        while let Some(&c) = self.chars.peek() {
            if !(c == '~'
                || c == '`'
                || c == '!'
                || c == '@'
                || c == '$'
                || c == '%'
                || c == '^'
                || c == '&'
                || c == '*'
                || c == '-'
                || c == '+'
                || c == '='
                || c == '|'
                || c == ':'
                || c == '<'
                || c == '>'
                || c == '.'
                || c == '?'
                || c == '/'
                || c == '\''
                || c == '_')
            {
                break;
            }
            name.push(c);
            self.advance_in_same_line();
        }

        match self.sym_kw_table.get(name.as_str()) {
            Some(keyword_token) => Token(keyword_token.to_owned(), Span(start_pos, self.pos)),
            None => Token(Id(name), Span(start_pos, self.pos)),
        }
    }

    /// Handles situations where the lookahead is `|`.
    fn lex_pipe(&mut self) -> Token {
        let start_pos = Pos(self.pos.0, self.pos.1 + 1);

        // This is for performing the second lookahead
        let mut temp_iter = self.chars.clone();
        temp_iter.next();
        match temp_iter.peek() {
            Some(')') => {
                self.advance_in_same_line();
                self.advance_in_same_line();
                Token(PipeRp, Span(start_pos, self.pos))
            }
            Some(']') => {
                self.advance_in_same_line();
                self.advance_in_same_line();
                Token(PipeRb, Span(start_pos, self.pos))
            }
            Some('}') => {
                self.advance_in_same_line();
                self.advance_in_same_line();
                Token(PipeRc, Span(start_pos, self.pos))
            }
            // Otherwise, this is just the beginning of something
            // like a symbolic identifier
            _ => self.lex_sym(),
        }
    }

    /// Handles situations where the lookahead is `.`.
    fn lex_dot(&mut self) -> Token {
        let start_pos = Pos(self.pos.0, self.pos.1 + 1);

        // This is for performing the second lookahead
        let mut temp_iter = self.chars.clone();
        temp_iter.next();
        match temp_iter.peek() {
            Some('[') => {
                self.advance_in_same_line();
                self.advance_in_same_line();
                Token(DotLp, Span(start_pos, self.pos))
            }
            // Otherwise, this is just the beginning of something
            // like a symbolic identifier
            _ => self.lex_sym(),
        }
    }

    /// Handles situations where the lookahead is `(`.
    fn lex_lp(&mut self) -> Token {
        let start_pos = Pos(self.pos.0, self.pos.1 + 1);
        self.advance_in_same_line();
        if let Some('|') = self.chars.peek() {
            self.advance_in_same_line();
            Token(LpPipe, Span(start_pos, self.pos))
        } else {
            Token(Lp, Span(start_pos, self.pos))
        }
    }

    /// Handles situations where the lookahead is `)`.
    fn lex_rp(&mut self) -> Token {
        let start_pos = Pos(self.pos.0, self.pos.1 + 1);
        self.advance_in_same_line();
        Token(Rp, Span(start_pos, self.pos))
    }

    /// Handles situations where the lookahead is `[`.
    fn lex_lb(&mut self) -> Token {
        let start_pos = Pos(self.pos.0, self.pos.1 + 1);
        self.advance_in_same_line();
        if let Some('|') = self.chars.peek() {
            self.advance_in_same_line();
            Token(LbPipe, Span(start_pos, self.pos))
        } else {
            Token(Lb, Span(start_pos, self.pos))
        }
    }

    /// Handles situations where the lookahead is `]`.
    fn lex_rb(&mut self) -> Token {
        let start_pos = Pos(self.pos.0, self.pos.1 + 1);
        self.advance_in_same_line();
        Token(Rb, Span(start_pos, self.pos))
    }

    /// Handles situations where the lookahead is `{`.
    fn lex_lc(&mut self) -> Token {
        let start_pos = Pos(self.pos.0, self.pos.1 + 1);
        self.advance_in_same_line();
        if let Some('|') = self.chars.peek() {
            self.advance_in_same_line();
            Token(LcPipe, Span(start_pos, self.pos))
        } else {
            Token(Lc, Span(start_pos, self.pos))
        }
    }

    /// Handles situations where the lookahead is `}`.
    fn lex_rc(&mut self) -> Token {
        let start_pos = Pos(self.pos.0, self.pos.1 + 1);
        self.advance_in_same_line();
        Token(Rc, Span(start_pos, self.pos))
    }

    /// Handles situations where the lookahead is `,`.
    fn lex_comma(&mut self) -> Token {
        let start_pos = Pos(self.pos.0, self.pos.1 + 1);
        self.advance_in_same_line();
        Token(Comma, Span(start_pos, self.pos))
    }

    /// Handles situations where the lookahead is `;`.
    fn lex_semicolon(&mut self) -> Token {
        let start_pos = Pos(self.pos.0, self.pos.1 + 1);
        self.advance_in_same_line();
        Token(ExprEnd, Span(start_pos, self.pos))
    }

    /// Handles situations where the lookahead is `\n`.
    fn lex_eol(&mut self) -> Token {
        self.advance_to_next_line();
        Token(ExprEnd, Span(self.pos, self.pos))
    }

    /// Handles situations where the lookahead is `\`.
    fn lex_backslash(&mut self) -> Token {
        let start_pos = Pos(self.pos.0, self.pos.1 + 1);
        self.advance_in_same_line();
        Token(ExprContinue, Span(start_pos, self.pos))
    }
}

/// The [`Lexer`] serves as an iterator generating [`Result<Token, Error>`]s.
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_ws();

        match self.chars.peek()? {
            '#' => {
                self.skip_line_comment();
                self.next()
            }
            '\'' => Some(self.lex_char_lit()),
            '"' => Some(self.lex_str_lit()),
            &c if c.is_ascii_digit() => Some(self.lex_num_lit()),
            &c if c.is_alphabetic() || c == '_' => Some(Ok(self.lex_alpha())),
            &c if c == '~'
                || c == '`'
                || c == '!'
                || c == '@'
                || c == '$'
                || c == '%'
                || c == '^'
                || c == '&'
                || c == '*'
                || c == '-'
                || c == '+'
                || c == '='
                || c == ':'
                || c == '<'
                || c == '>'
                || c == '?'
                || c == '/' =>
            {
                Some(Ok(self.lex_sym()))
            }
            // '|' is left out from the branch above and handled
            // specially, since it can lead "|)", "|]", and "}"
            '|' => Some(Ok(self.lex_pipe())),
            // '.' is left out from the branch above and handled
            // specially as well, since it can lead ".["
            '.' => Some(Ok(self.lex_dot())),
            '(' => Some(Ok(self.lex_lp())),
            ')' => Some(Ok(self.lex_rp())),
            '[' => Some(Ok(self.lex_lb())),
            ']' => Some(Ok(self.lex_rb())),
            '{' => Some(Ok(self.lex_lc())),
            '}' => Some(Ok(self.lex_rc())),
            ',' => Some(Ok(self.lex_comma())),
            ';' => Some(Ok(self.lex_semicolon())),
            '\n' => Some(Ok(self.lex_eol())),
            '\\' => Some(Ok(self.lex_backslash())),

            // The lookahead cannot be lexed
            _ => {
                self.advance_in_same_line();
                Some(Err(Error::UnexpectedChar(self.pos)))
            }
        }
    }
}
