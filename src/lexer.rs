use std::{iter::Peekable, str::Chars};

use crate::{
    error::{Error, ErrorKind::*},
    token::{Pos, Span, Token, TokenKind::*},
};

/// Characters allowed in symbolic names.
const SYM_CHARS: &str = "~`!@#$%^&*-+=|\\:'<,>.?/";

/// Lexer for a single line of Lynx source.
///
/// Since no Lynx token spans multiple lines,
/// the overall lexing task can be divided into independent per-line passes.
/// This type is an internal helper for [`tokenize`]
/// and is *not* intended for public use.
struct LineLexer<'a> {
    /// Peekable iterator over the characters in the line.
    chars: Peekable<Chars<'a>>,

    /// Line number, `1`-based.
    line_no: usize,

    /// Column number *before* the lookahead;
    /// starts at `0` before any character is consumed,
    /// thus still `1`-based.
    col_no: usize,
}

impl<'a> LineLexer<'a> {
    /// Creates [`LineLexer`] from a single line of Lynx source
    /// and the line number.
    fn new(src: &'a str, line_no: usize) -> Self {
        Self {
            chars: src.chars().peekable(),
            line_no,
            col_no: 0,
        }
    }

    /// Advances lexer state by incrementing [`Self::col_no`]
    /// and consuming one character from [`Self::chars`].
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

    /// Handles escape sequence in a character/string literal,
    /// invoked when the lookahead is `\`.
    fn handle_esc_seq(&mut self, lit_start_pos: Pos) -> Result<char, Error> {
        self.advance(); // Skip `\`
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

            // Unicode escape sequence: `\u{...}`
            Some('u') => {
                self.advance();

                if let Some('{') = self.chars.peek() {
                    self.advance();
                } else {
                    self.advance(); // Skip invalid character
                    return Err(Error(UnknownEscapeSeq, Span(esc_start_pos, self.pos())));
                }

                let mut hex_str = String::new();
                loop {
                    match self.chars.peek() {
                        Some('}') => {
                            self.advance();
                            break;
                        }
                        Some(&c) if c.is_ascii_hexdigit() => {
                            self.advance();
                            hex_str.push(c);
                        }
                        Some(_) => {
                            self.advance(); // Skip invalid character
                            return Err(Error(UnknownEscapeSeq, Span(esc_start_pos, self.pos())));
                        }
                        None => {
                            return Err(Error(
                                UnterminatedCharOrStrLit,
                                Span(lit_start_pos, self.pos()),
                            ));
                        }
                    }
                }

                let code_point = u32::from_str_radix(&hex_str, 16)
                    .map_err(|_| Error(UnknownEscapeSeq, Span(esc_start_pos, self.pos())))?;
                char::from_u32(code_point)
                    .ok_or_else(|| Error(UnknownEscapeSeq, Span(esc_start_pos, self.pos())))?
            }

            Some(_) => {
                self.advance(); // Skip invalid character
                return Err(Error(UnknownEscapeSeq, Span(esc_start_pos, self.pos())));
            }
            None => {
                return Err(Error(
                    UnterminatedCharOrStrLit,
                    Span(lit_start_pos, self.pos()),
                ));
            }
        };

        Ok(escaped_ch)
    }

    /// Lexes character literals,
    /// invoked when the lookahead is `'`.
    fn lex_char_lit(&mut self) -> Result<Token, Error> {
        self.advance(); // Skip `'`
        let start_pos = self.pos();
        let mut ch_vec = Vec::new();

        loop {
            match self.chars.peek() {
                Some('\'') => {
                    self.advance();
                    match ch_vec.len() {
                        0 => {
                            return Err(Error(EmptyCharLit, Span(start_pos, self.pos())));
                        }
                        1 => {
                            return Ok(Token(CharLit(ch_vec[0]), Span(start_pos, self.pos())));
                        }
                        _ => {
                            return Err(Error(MultipleCharsInCharLit, Span(start_pos, self.pos())));
                        }
                    }
                }

                Some('\\') => {
                    // Escape sequence
                    let escaped_ch = self.handle_esc_seq(start_pos)?;
                    ch_vec.push(escaped_ch);
                }

                Some(&c) => {
                    self.advance();
                    ch_vec.push(c);
                }

                None => {
                    return Err(Error(UnterminatedCharOrStrLit, Span(start_pos, self.pos())));
                }
            }
        }
    }

    /// Lexes quoted string literals,
    /// invoked when the lookahead is `"`.
    fn lex_quoted_str_lit(&mut self) -> Result<Token, Error> {
        self.advance(); // Skip `"`
        let start_pos = self.pos();
        let mut s = String::new();

        loop {
            match self.chars.peek() {
                Some('"') => {
                    self.advance();
                    return Ok(Token(StrLit(s), Span(start_pos, self.pos())));
                }

                Some('\\') => {
                    // Escape sequence
                    let escaped_ch = self.handle_esc_seq(start_pos)?;
                    s.push(escaped_ch);
                }

                Some(&c) => {
                    self.advance();
                    s.push(c);
                }

                None => {
                    return Err(Error(UnterminatedCharOrStrLit, Span(start_pos, self.pos())));
                }
            }
        }
    }

    /// Lexes raw string literals,
    /// invoked when the lookahead is `\\`.
    fn lex_raw_string_lit(&mut self) -> Token {
        self.advance(); // Skip first `\`
        let start_pos = self.pos();
        self.advance(); // Skip second `\`
        let mut s = String::new();

        while let Some(&c) = self.chars.peek() {
            self.advance();
            s.push(c);
        }

        Token(StrLit(s), Span(start_pos, self.pos()))
    }

    /// Checks if a character is a valid digit under the given base,
    /// which is among `2`, `8`, `10`, or `16`.
    fn is_valid_digit(c: char, base: u32) -> bool {
        match base {
            2 => c == '0' || c == '1',
            8 => c.is_ascii_digit() && c < '8',
            10 => c.is_ascii_digit(),
            16 => c.is_ascii_hexdigit(),
            _ => false,
        }
    }

    /// Lexes number literals,
    /// invoked when the lookahead is an ASCII digit.
    fn lex_num_lit(&mut self, lookahead: char) -> Result<Token, Error> {
        self.advance();
        let start_pos = self.pos();
        let mut num_str = String::new();

        let mut is_float = false;
        let mut base = 10;

        // Check for base prefixes
        if lookahead == '0' {
            match self.chars.peek() {
                Some('x' | 'X') => {
                    self.advance();
                    base = 16;
                }
                Some('b' | 'B') => {
                    self.advance();
                    base = 2;
                }
                Some('o' | 'O') => {
                    self.advance();
                    base = 8;
                }
                _ => {
                    // Just a decimal number starting with `0`
                    num_str.push(lookahead);
                }
            }
        } else {
            num_str.push(lookahead);
        }

        while let Some(&c) = self.chars.peek() {
            match c {
                '_' => {
                    self.advance(); // Skip `_` in number literals
                }
                '.' if base == 10 => {
                    self.advance();
                    // Only decimal numbers can be floating-point
                    if is_float {
                        break;
                    }
                    is_float = true;
                    num_str.push('.');
                }
                c if Self::is_valid_digit(c, base) => {
                    self.advance();
                    num_str.push(c);
                }
                _ => {
                    break;
                }
            }
        }

        // Parse the number
        if is_float {
            if let Ok(num) = num_str.parse::<f64>() {
                Ok(Token(FloatLit(num), Span(start_pos, self.pos())))
            } else {
                Err(Error(InvalidNumLitFormat, Span(start_pos, self.pos())))
            }
        } else {
            if let Ok(num) = i64::from_str_radix(&num_str, base) {
                Ok(Token(IntLit(num), Span(start_pos, self.pos())))
            } else {
                Err(Error(InvalidNumLitFormat, Span(start_pos, self.pos())))
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
            self.advance();
            name.push(c);
        }

        Token(Name(name), Span(start_pos, self.pos()))
    }

    /// Lexes symbolic names,
    /// invoked when the lookahead is among [`SYM_CHARS`]
    /// excluding `-`, `\`, and `'`.
    fn lex_sym(&mut self, lookahead: char) -> Token {
        self.advance();
        let start_pos = self.pos();
        let mut name = String::new();
        name.push(lookahead);

        while let Some(&c) = self.chars.peek() {
            if !SYM_CHARS.contains(c) {
                break;
            }
            self.advance();
            name.push(c);
        }

        Token(Name(name), Span(start_pos, self.pos()))
    }

    /// Handles lookahead `(`.
    fn lex_lp(&mut self) -> Token {
        self.advance();
        match self.chars.peek() {
            // `()`: unit literal
            Some(')') => {
                let start_pos = self.pos();
                self.advance();
                Token(UnitLit, Span(start_pos, self.pos()))
            }
            // Otherwise: just a left parenthesis
            _ => Token(Lp, Span(self.pos(), self.pos())),
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

    /// Handles lookahead `-`,
    /// returning [`None`] if a line comment is encountered.
    fn lex_hyphen(&mut self) -> Option<Token> {
        // Cloned to perform a second lookahead
        match self.chars.clone().nth(1) {
            // `--`: line comment
            Some('-') => {
                self.skip_line();
                None
            }
            // Otherwise: just a symbolic name
            _ => Some(self.lex_sym('-')),
        }
    }

    /// Handles lookahead `\`.
    fn lex_backslash(&mut self) -> Token {
        // Cloned to perform a second lookahead
        match self.chars.clone().nth(1) {
            // `\\`: raw string literal
            Some('\\') => self.lex_raw_string_lit(),
            // Otherwise: just a symbolic name
            _ => self.lex_sym('\\'),
        }
    }

    /// Handles unknown lookahead.
    fn lex_unknown(&mut self) -> Error {
        self.advance();
        Error(UnexpectedChar, Span(self.pos(), self.pos()))
    }

    /// Lexes the line, returning either a [`Vec`] of all [`Token`]s
    /// or the first [`Error`] encountered.
    pub fn tokenize(mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();
        loop {
            self.skip_ws();

            match self.chars.peek() {
                None => {
                    break;
                }

                Some(&c) => {
                    let token = match c {
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
                        c if SYM_CHARS.contains(c) => self.lex_sym(c),
                        _ => {
                            return Err(self.lex_unknown());
                        }
                    };
                    tokens.push(token);
                }
            }
        }

        Ok(tokens)
    }
}

/// Lexes Lynx source, returning either a [`Vec`] of all [`Token`]s
/// or the first [`Error`] encountered.
pub fn tokenize(src: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();
    for (line_idx, line_str) in src.lines().enumerate() {
        let line_no = line_idx + 1;
        let line_lexer = LineLexer::new(line_str, line_no);
        let line_tokens = line_lexer.tokenize()?;
        tokens.extend(line_tokens);
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenKind;

    fn token_kinds(tokens: Vec<Token>) -> Vec<TokenKind> {
        tokens.into_iter().map(|Token(kind, _)| kind).collect()
    }

    #[test]
    fn test_empty_line() {
        let tokens = tokenize("").unwrap();
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_whitespace_only() {
        let tokens = tokenize("   \t  ").unwrap();
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_basic_delimiters() {
        let tokens = tokenize("( ) [ ] { } ;").unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(kinds, vec![Lp, Rp, Lb, Rb, Lc, Rc, Semicolon]);
    }

    #[test]
    fn test_unit_literal() {
        let tokens = tokenize("()").unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(kinds, vec![UnitLit]);
    }

    #[test]
    fn test_unit_with_space() {
        let tokens = tokenize("( )").unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(kinds, vec![Lp, Rp]);
    }

    #[test]
    fn test_integer_literals() {
        let tokens = tokenize("0 42 999").unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(kinds, vec![IntLit(0), IntLit(42), IntLit(999)]);
    }

    #[test]
    fn test_float_literals() {
        let tokens = tokenize("3.14 0.5 100.0").unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(kinds, vec![FloatLit(3.14), FloatLit(0.5), FloatLit(100.0)]);
    }

    #[test]
    fn test_alphabetic_names() {
        let tokens = tokenize("foo bar_baz qux123 test'").unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(
            kinds,
            vec![
                Name("foo".to_string()),
                Name("bar_baz".to_string()),
                Name("qux123".to_string()),
                Name("test'".to_string())
            ]
        );
    }

    #[test]
    fn test_symbolic_names() {
        let tokens = tokenize("+ ++ <> :: =>").unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(
            kinds,
            vec![
                Name("+".to_string()),
                Name("++".to_string()),
                Name("<>".to_string()),
                Name("::".to_string()),
                Name("=>".to_string())
            ]
        );
    }

    #[test]
    fn test_line_comment() {
        let tokens = tokenize("foo -- this is a comment").unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(kinds, vec![Name("foo".to_string())]);
    }

    #[test]
    fn test_double_hyphen_comment() {
        let tokens = tokenize("-- entire line comment").unwrap();
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_char_literal_simple() {
        let tokens = tokenize("'a' 'Z' '0'").unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(kinds, vec![CharLit('a'), CharLit('Z'), CharLit('0')]);
    }

    #[test]
    fn test_char_literal_escape_sequences() {
        let tokens = tokenize(r"'\n' '\r' '\t' '\\' '\0'").unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(
            kinds,
            vec![
                CharLit('\n'),
                CharLit('\r'),
                CharLit('\t'),
                CharLit('\\'),
                CharLit('\0')
            ]
        );
    }

    #[test]
    fn test_char_literal_quote_escapes() {
        let tokens = tokenize(r#"'\'' '\"'"#).unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(kinds, vec![CharLit('\''), CharLit('"')]);
    }

    #[test]
    fn test_char_literal_unicode_escape() {
        let tokens = tokenize(r"'\u{41}' '\u{1F600}' '\u{3B1}'").unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(kinds, vec![CharLit('A'), CharLit('😀'), CharLit('α')]);
    }

    #[test]
    fn test_empty_char_literal_error() {
        let result = tokenize("''");
        assert!(matches!(result, Err(Error(EmptyCharLit, _))));
    }

    #[test]
    fn test_multiple_chars_in_char_literal_error() {
        let result = tokenize("'ab'");
        assert!(matches!(result, Err(Error(MultipleCharsInCharLit, _))));
    }

    #[test]
    fn test_unterminated_char_literal_error() {
        let result = tokenize("'a");
        assert!(matches!(result, Err(Error(UnterminatedCharOrStrLit, _))));
    }

    #[test]
    fn test_string_literal_simple() {
        let tokens = tokenize(r#""hello" "world""#).unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(
            kinds,
            vec![StrLit("hello".to_string()), StrLit("world".to_string())]
        );
    }

    #[test]
    fn test_string_literal_empty() {
        let tokens = tokenize(r#""""#).unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(kinds, vec![StrLit("".to_string())]);
    }

    #[test]
    fn test_string_literal_with_escapes() {
        let tokens = tokenize(r#""line1\nline2\ttab\0null""#).unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(kinds, vec![StrLit("line1\nline2\ttab\0null".to_string())]);
    }

    #[test]
    fn test_string_literal_with_unicode_escape() {
        let tokens = tokenize(r#""\u{48}\u{65}\u{6C}\u{6C}\u{6F}""#).unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(kinds, vec![StrLit("Hello".to_string())]);
    }

    #[test]
    fn test_raw_string_literal() {
        let tokens = tokenize(r"\\raw\nstring\twith\escapes").unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(
            kinds,
            vec![StrLit(r"raw\nstring\twith\escapes".to_string())]
        );
    }

    #[test]
    fn test_unterminated_string_literal_error() {
        let result = tokenize(r#""unterminated"#);
        assert!(matches!(result, Err(Error(UnterminatedCharOrStrLit, _))));
    }

    #[test]
    fn test_unknown_escape_sequence_error() {
        let result = tokenize(r"'\x'");
        assert!(matches!(result, Err(Error(UnknownEscapeSeq, _))));
    }

    #[test]
    fn test_invalid_unicode_escape_no_brace() {
        let result = tokenize(r"'\u41'");
        assert!(matches!(result, Err(Error(UnknownEscapeSeq, _))));
    }

    #[test]
    fn test_invalid_unicode_escape_empty() {
        let result = tokenize(r"'\u{}'");
        assert!(matches!(result, Err(Error(UnknownEscapeSeq, _))));
    }

    #[test]
    fn test_invalid_unicode_escape_bad_hex() {
        let result = tokenize(r"'\u{XYZ}'");
        assert!(matches!(result, Err(Error(UnknownEscapeSeq, _))));
    }

    #[test]
    fn test_invalid_unicode_escape_invalid_codepoint() {
        let result = tokenize(r"'\u{FFFFFF}'");
        assert!(matches!(result, Err(Error(UnknownEscapeSeq, _))));
    }

    #[test]
    fn test_unexpected_char_error() {
        let result = tokenize("§");
        assert!(matches!(result, Err(Error(UnexpectedChar, _))));
    }

    #[test]
    fn test_mixed_tokens() {
        let tokens = tokenize(r#"foo 42 "bar" 'x' (baz)"#).unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(
            kinds,
            vec![
                Name("foo".to_string()),
                IntLit(42),
                StrLit("bar".to_string()),
                CharLit('x'),
                Lp,
                Name("baz".to_string()),
                Rp
            ]
        );
    }

    #[test]
    fn test_multiline_tokenize() {
        let src = "foo\nbar\nbaz";
        let tokens = tokenize(src).unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(
            kinds,
            vec![
                Name("foo".to_string()),
                Name("bar".to_string()),
                Name("baz".to_string())
            ]
        );
    }

    #[test]
    fn test_hyphen_in_symbolic_name() {
        let tokens = tokenize("-").unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(kinds, vec![Name("-".to_string())]);
    }

    #[test]
    fn test_backslash_in_symbolic_name() {
        let tokens = tokenize(r"\").unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(kinds, vec![Name(r"\".to_string())]);
    }

    #[test]
    fn test_binary_literals() {
        let tokens = tokenize("0b1010 0b1111_0000 0B101").unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(
            kinds,
            vec![IntLit(0b1010), IntLit(0b1111_0000), IntLit(0b101)]
        );
    }

    #[test]
    fn test_octal_literals() {
        let tokens = tokenize("0o755 0o7_7_7 0O10").unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(kinds, vec![IntLit(0o755), IntLit(0o777), IntLit(0o10)]);
    }

    #[test]
    fn test_hex_literals() {
        let tokens = tokenize("0xFF 0xDEAD_BEEF 0X10").unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(kinds, vec![IntLit(0xFF), IntLit(0xDEAD_BEEF), IntLit(0x10)]);
    }

    #[test]
    fn test_underscores_in_decimals() {
        let tokens = tokenize("1_000_000 1_2_3").unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(kinds, vec![IntLit(1_000_000), IntLit(123)]);
    }

    #[test]
    fn test_underscores_in_floats() {
        let tokens = tokenize("1_000.5 3_14.15_92").unwrap();
        let kinds = token_kinds(tokens);
        assert_eq!(kinds, vec![FloatLit(1000.5), FloatLit(314.1592)]);
    }

    #[test]
    fn test_invalid_base_prefix_no_digits() {
        let result = tokenize("0x");
        assert!(matches!(result, Err(Error(InvalidNumLitFormat, _))));
    }

    #[test]
    fn test_invalid_binary_digit() {
        let result = tokenize("0b102");
        let tokens = result.unwrap();
        let kinds = token_kinds(tokens);
        // Should parse 0b10 and then 2 separately
        assert_eq!(kinds, vec![IntLit(0b10), IntLit(2)]);
    }
}
