use std::{collections::HashSet, iter::Peekable, str::Chars};

use crate::{
    error::Error,
    token::{Pos, Span, Token, TokenKind::*},
};

/// Lexer for a single line of Lynx source.
///
/// This type is an internal helper used by the top-level [`Lexer`] and is *not*
/// intended for public use. Since no Lynx token spans multiple lines,
/// the overall lexing task can be divided into independent per-line passes,
/// each handled by a [`LineLexer`], which simplifies lexing logic.
struct LineLexer<'a> {
    /// Peekable iterator over the line.
    chars: Peekable<Chars<'a>>,

    /// Current line number.
    line_no: usize,

    /// Current column number (before the lookahead).
    col_no: usize,

    /// Set of characters that are allowed in symbolic names.
    sym_char_set: HashSet<char>,
}

impl<'a> LineLexer<'a> {
    /// Creates [`LineLexer`] from a single line of Lynx source
    /// and the line number.
    fn new(src: &'a str, line_no: usize) -> Self {
        Self {
            chars: src.chars().peekable(),
            line_no,
            col_no: 0,

            sym_char_set: HashSet::from([
                '~', '`', '!', '@', '#', '$', '%', '^', '&', '*', '-', '+', '=', '|', '\\', ':',
                '\'', '<', ',', '>', '.', '?', '/',
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

    /// Skips whitespace.
    fn skip_ws(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    /// Skips the rest of the line,
    /// invoked when the lookahead is `--`.
    fn skip_line(&mut self) {
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
                            return Err(Error::EmptyCharLit(Span(start_pos, self.pos())));
                        }
                        1 => {
                            return Ok(Token(CharLit(ch_vec[0]), Span(start_pos, self.pos())));
                        }
                        _ => {
                            return Err(Error::MultipleCharsInCharLit(Span(start_pos, self.pos())));
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
                            return Err(Error::UnknownEscapeSeq(Span(esc_start_pos, self.pos())));
                        }
                        None => {
                            return Err(Error::UnterminatedCharLit(Span(start_pos, self.pos())));
                        }
                    };

                    ch_vec.push(escaped_ch);
                }

                Some(&c) => {
                    self.advance();
                    ch_vec.push(c);
                }

                None => {
                    return Err(Error::UnterminatedCharLit(Span(start_pos, self.pos())));
                }
            }
        }
    }

    /// Lexes quoted string literals,
    /// invoked when the lookahead is `"`.
    fn lex_quoted_str_lit(&mut self) -> Result<Token, Error> {
        self.advance(); // Skip opening quote
        let start_pos = self.pos();
        let mut s = String::new();

        loop {
            match self.chars.peek() {
                Some('"') => {
                    // Closing quote
                    self.advance();
                    return Ok(Token(StrLit(s), Span(start_pos, self.pos())));
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
                            return Err(Error::UnknownEscapeSeq(Span(esc_start_pos, self.pos())));
                        }
                        None => {
                            return Err(Error::UnterminatedStrLit(Span(start_pos, self.pos())));
                        }
                    };

                    s.push(escaped_ch);
                }

                Some(&c) => {
                    self.advance();
                    s.push(c);
                }

                None => {
                    return Err(Error::UnterminatedStrLit(Span(start_pos, self.pos())));
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

        Token(StrLit(s), Span(start_pos, self.pos()))
    }

    // TODO: Support more number formats, like base prefixes and underscores
    /// Lexes number literals,
    /// invoked when the lookahead is an ASCII digit.
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
                Ok(Token(FloatLit(num), Span(start_pos, self.pos())))
            } else {
                Err(Error::InvalidNumLitFormat(Span(start_pos, self.pos())))
            }
        } else {
            if let Ok(num) = num_str.parse::<i64>() {
                Ok(Token(IntLit(num), Span(start_pos, self.pos())))
            } else {
                Err(Error::InvalidNumLitFormat(Span(start_pos, self.pos())))
            }
        }
    }

    /// Lexes alphabetic names,
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

        Token(Name(name), Span(start_pos, self.pos()))
    }

    /// Lexes symbolic names,
    /// invoked when the lookahead is among [`Self::sym_char_set`]
    /// excluding `-`, `\`, and `'`.
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

        Token(Name(name), Span(start_pos, self.pos()))
    }

    /// Handles lookahead `(`.
    fn lex_lp(&mut self) -> Token {
        // This is for performing the second lookahead
        let mut temp_iter = self.chars.clone();
        temp_iter.next();
        match temp_iter.peek() {
            // `()`: unit literal
            Some(')') => {
                self.advance();
                let start_pos = self.pos();
                self.advance();
                Token(UnitLit, Span(start_pos, self.pos()))
            }
            // Otherwise: just a left parenthesis
            _ => {
                self.advance();
                Token(Lp, Span(self.pos(), self.pos()))
            }
        }
    }

    /// Handles lookahead `)`.
    fn lex_rp(&mut self) -> Token {
        self.advance();
        Token(Rp, Span(self.pos(), self.pos()))
    }

    /// Handles lookahead `[`.
    fn lex_lb(&mut self) -> Token {
        self.advance();
        Token(Lb, Span(self.pos(), self.pos()))
    }

    /// Handles lookahead `]`.
    fn lex_rb(&mut self) -> Token {
        self.advance();
        Token(Rb, Span(self.pos(), self.pos()))
    }

    /// Handles lookahead `{`.
    fn lex_lc(&mut self) -> Token {
        self.advance();
        Token(Lc, Span(self.pos(), self.pos()))
    }

    /// Handles lookahead `}`.
    fn lex_rc(&mut self) -> Token {
        self.advance();
        Token(Rc, Span(self.pos(), self.pos()))
    }

    /// Handles lookahead `;`.
    fn lex_semicolon(&mut self) -> Token {
        self.advance();
        Token(Semicolon, Span(self.pos(), self.pos()))
    }

    /// Handles lookahead `-`.
    fn lex_hyphen(&mut self) -> Option<Token> {
        // This is for performing the second lookahead
        let mut temp_iter = self.chars.clone();
        temp_iter.next();
        match temp_iter.peek() {
            // `--`: line comment
            Some('-') => {
                self.skip_line();
                None
            }
            // Otherwise: the beginning of a symbolic name
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
            // Otherwise: the beginning of a symbolic name
            _ => self.lex_sym('\\'),
        }
    }

    fn lex_unknown(&mut self) -> Error {
        self.advance();
        Error::UnexpectedChar(Span(self.pos(), self.pos()))
    }
}

impl<'a> LineLexer<'a> {
    /// Lexes the line, returns either a [TokenStream] of all [Token]s
    /// or the first [Error] encountered.
    pub fn tokenize(mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();
        loop {
            self.skip_ws();

            match self.chars.peek() {
                None => {
                    break;
                }
                Some(&c) => {
                    let result = match c {
                        '(' => self.lex_lp(),
                        ')' => self.lex_rp(),
                        '[' => self.lex_lb(),
                        ']' => self.lex_rb(),
                        '{' => self.lex_lc(),
                        '}' => self.lex_rc(),
                        ';' => self.lex_semicolon(),
                        '-' => match self.lex_hyphen() {
                            Some(token) => token,
                            None => break,
                        },
                        '\\' => self.lex_backslash(),
                        '\'' => self.lex_char_lit()?,
                        '"' => self.lex_quoted_str_lit()?,
                        c if c.is_ascii_digit() => self.lex_num_lit(c)?,
                        c if c.is_alphabetic() || c == '_' => self.lex_alpha(c),
                        c if self.sym_char_set.contains(&c) => self.lex_sym(c),
                        _ => {
                            return Err(self.lex_unknown());
                        }
                    };
                    tokens.push(result);
                }
            }
        }
        Ok(tokens)
    }
}

/// Top-level lexer for Lynx source.
pub struct Lexer<'a> {
    /// The source to be lexed.
    src: &'a str,
}

impl<'a> Lexer<'a> {
    /// Creates [`Lexer`] from Lynx source.
    pub fn new(src: &'a str) -> Self {
        Self { src }
    }

    /// Lexes the source, returns either a [TokenStream] of all [Token]s
    /// or the first [Error] encountered.
    pub fn tokenize(self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();
        for (line_idx, line_str) in self.src.lines().enumerate() {
            let line_no = line_idx + 1;
            let line_lexer = LineLexer::new(line_str, line_no);
            let line_stream = line_lexer.tokenize()?;
            tokens.extend(line_stream);
        }
        Ok(tokens)
    }
}
