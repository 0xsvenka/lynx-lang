use std::iter::Peekable;

use crate::{error::Error, expr::Expr, lexer::Lexer, token::Token};

pub struct Parser<'a> {
    tokens: Peekable<Lexer<'a>>,
    current_token: Option<Token>,
    line: usize,
    col: usize,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser {
            tokens: lexer.peekable(),
            current_token: None,
            line: 1,
            col: 1,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.chars.next();
        if c == Some('\n') {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        c
    }

    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.next_char();
            } else if c == '-' {
                let start_col = self.col;
                self.next_char();
                if let Some('-') = self.peek_char() {
                    // Line comment
                    self.next_char();
                    while let Some(c) = self.next_char() {
                        if c == '\n' {
                            break;
                        }
                    }
                } else {
                    // Not a comment, put back the '-'
                    self.col = start_col;
                    self.chars = std::iter::once('-').chain(self.chars.by_ref().cloned()).peekable();
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self, first: char) -> String {
        let mut ident = String::new();
        ident.push(first);
        
        while let Some(&c) = self.peek_char() {
            if c.is_alphanumeric() || c == '\'' || c == '_' {
                ident.push(self.next_char().unwrap());
            } else {
                break;
            }
        }
        
        ident
    }

    fn read_number(&mut self, first: char) -> Result<Token, ParseError> {
        let mut num_str = String::new();
        num_str.push(first);
        let mut is_float = false;
        
        while let Some(&c) = self.peek_char() {
            if c.is_ascii_digit() {
                num_str.push(self.next_char().unwrap());
            } else if c == '.' {
                if is_float {
                    return Err(ParseError::InvalidToken("Invalid number format".to_string()));
                }
                is_float = true;
                num_str.push(self.next_char().unwrap());
            } else {
                break;
            }
        }
        
        if is_float {
            num_str.parse::<f64>()
                .map(Token::Float)
                .map_err(|_| ParseError::InvalidToken("Invalid float literal".to_string()))
        } else {
            num_str.parse::<i64>()
                .map(Token::Int)
                .map_err(|_| ParseError::InvalidToken("Invalid integer literal".to_string()))
        }
    }

    fn read_string(&mut self) -> Result<Token, ParseError> {
        let mut s = String::new();
        while let Some(c) = self.next_char() {
            if c == '"' {
                return Ok(Token::String(s));
            } else if c == '\\' {
                // Handle escape sequences
                if let Some(esc) = self.next_char() {
                    s.push(match esc {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '"' => '"',
                        '\'' => '\'',
                        _ => return Err(ParseError::InvalidToken(format!("Unknown escape sequence \\{}", esc))),
                    });
                } else {
                    return Err(ParseError::UnexpectedEOF);
                }
            } else {
                s.push(c);
            }
        }
        Err(ParseError::UnexpectedEOF)
    }

    fn read_char(&mut self) -> Result<Token, ParseError> {
        let c = self.next_char().ok_or(ParseError::UnexpectedEOF)?;
        if c == '\\' {
            // Handle escape sequences
            let esc = self.next_char().ok_or(ParseError::UnexpectedEOF)?;
            let esc_char = match esc {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                '\\' => '\\',
                '\'' => '\'',
                '"' => '"',
                _ => return Err(ParseError::InvalidToken(format!("Unknown escape sequence \\{}", esc))),
            };
            if self.next_char() != Some('\'') {
                return Err(ParseError::InvalidToken("Unterminated character literal".to_string()));
            }
            Ok(Token::Char(esc_char))
        } else {
            if self.next_char() != Some('\'') {
                return Err(ParseError::InvalidToken("Unterminated character literal".to_string()));
            }
            Ok(Token::Char(c))
        }
    }

    fn next_token(&mut self) -> Result<Token, ParseError> {
        self.skip_whitespace();
        
        let c = match self.next_char() {
            Some(c) => c,
            None => return Ok(Token::EOF),
        };
        
        match c {
            '(' => Ok(Token::LParen),
            ')' => Ok(Token::RParen),
            '[' => Ok(Token::LBracket),
            ']' => Ok(Token::RBracket),
            ',' => Ok(Token::Comma),
            ';' => Ok(Token::Semicolon),
            ':' => {
                if self.peek_char() == Some(':') {
                    self.next_char();
                    Ok(Token::DoubleColon)
                } else {
                    Ok(Token::Colon)
                }
            }
            '=' => {
                if self.peek_char() == Some('>') {
                    self.next_char();
                    Ok(Token::Arrow)
                } else {
                    Ok(Token::Equals)
                }
            }
            '\\' => Ok(Token::Backslash),
            '.' => Ok(Token::Dot),
            '_' => Ok(Token::Underscore),
            '"' => self.read_string(),
            '\'' => self.read_char(),
            '-' if self.peek_char().map(|c| c.is_ascii_digit()).unwrap_or(false) => {
                let num = self.read_number('-')?;
                match num {
                    Token::Int(i) => Ok(Token::Int(-i)),
                    Token::Float(f) => Ok(Token::Float(-f)),
                    _ => unreachable!(),
                }
            }
            c if c.is_ascii_digit() => self.read_number(c),
            c if c.is_ascii_lowercase() || c == '_' => {
                let ident = self.read_identifier(c);
                match ident.as_str() {
                    "module" | "import" | "data" | "where" | "let" | "in" | "case" | "of" | "if" | "then" | "else" | "qualified" | "as" => {
                        Ok(Token::Keyword(ident))
                    }
                    "True" | "False" => Ok(Token::Ident(ident)),
                    _ => Ok(Token::Ident(ident)),
                }
            }
            c if c.is_ascii_uppercase() => {
                Ok(Token::ConIdent(self.read_identifier(c)))
            }
            c => Ok(Token::Symbol(c.to_string())),
        }
    }

    fn peek_token(&mut self) -> Result<&Token, ParseError> {
        if self.current_token.is_none() {
            self.current_token = Some(self.next_token()?);
        }
        Ok(self.current_token.as_ref().unwrap())
    }

    fn consume_token(&mut self) -> Result<Token, ParseError> {
        if let Some(token) = self.current_token.take() {
            Ok(token)
        } else {
            self.next_token()
        }
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), ParseError> {
        let token = self.consume_token()?;
        if token == expected {
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(format!("Expected {:?}, got {:?}", expected, token)))
        }
    }

    fn expect_ident(&mut self) -> Result<String, ParseError> {
        match self.consume_token()? {
            Token::Ident(s) => Ok(s),
            t => Err(ParseError::UnexpectedToken(format!("Expected identifier, got {:?}", t))),
        }
    }

    fn expect_con_ident(&mut self) -> Result<String, ParseError> {
        match self.consume_token()? {
            Token::ConIdent(s) => Ok(s),
            t => Err(ParseError::UnexpectedToken(format!("Expected constructor, got {:?}", t))),
        }
    }

    fn expect_keyword(&mut self, kw: &str) -> Result<(), ParseError> {
        match self.consume_token()? {
            Token::Keyword(s) if s == kw => Ok(()),
            t => Err(ParseError::UnexpectedToken(format!("Expected keyword '{}', got {:?}", kw, t))),
        }
    }

    fn parse_literal(&mut self) -> Result<Literal, ParseError> {
        match self.consume_token()? {
            Token::Int(i) => Ok(Literal::Int(i)),
            Token::Float(f) => Ok(Literal::Float(f)),
            Token::Char(c) => Ok(Literal::Char(c)),
            Token::String(s) => Ok(Literal::String(s)),
            Token::Ident(s) if s == "True" => Ok(Literal::Bool(true)),
            Token::Ident(s) if s == "False" => Ok(Literal::Bool(false)),
            t => Err(ParseError::UnexpectedToken(format!("Expected literal, got {:?}", t))),
        }
    }

    fn parse_type_var(&mut self) -> Result<Type, ParseError> {
        match self.consume_token()? {
            Token::Ident(s) => Ok(Type::TyVar(s)),
            t => Err(ParseError::UnexpectedToken(format!("Expected type variable, got {:?}", t))),
        }
    }

    fn parse_type_con(&mut self) -> Result<Type, ParseError> {
        match self.consume_token()? {
            Token::ConIdent(s) => Ok(Type::TyCon(s)),
            t => Err(ParseError::UnexpectedToken(format!("Expected type constructor, got {:?}", t))),
        }
    }

    fn parse_type_atom(&mut self) -> Result<Type, ParseError> {
        match self.peek_token()? {
            Token::LParen => {
                self.consume_token()?;
                let mut types = vec![self.parse_type()?];
                while self.peek_token()? == &Token::Comma {
                    self.consume_token()?;
                    types.push(self.parse_type()?);
                }
                self.expect_token(Token::RParen)?;
                if types.len() == 1 {
                    Ok(types.remove(0))
                } else {
                    Ok(Type::TyTuple(types))
                }
            }
            Token::LBracket => {
                self.consume_token()?;
                let typ = self.parse_type()?;
                self.expect_token(Token::RBracket)?;
                Ok(Type::TyList(Box::new(typ)))
            }
            Token::ConIdent(_) => self.parse_type_con(),
            Token::Ident(_) => self.parse_type_var(),
            t => Err(ParseError::UnexpectedToken(format!("Expected type, got {:?}", t))),
        }
    }

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        let mut left = self.parse_type_atom()?;
        
        // Handle function types
        while self.peek_token()? == &Token::Arrow {
            self.consume_token()?;
            let right = self.parse_type()?;
            left = Type::TyFun(Box::new(left), Box::new(right));
        }
        
        Ok(left)
    }

    fn parse_pattern_atom(&mut self) -> Result<Pattern, ParseError> {
        match self.peek_token()? {
            Token::LParen => {
                self.consume_token()?;
                let mut patterns = vec![self.parse_pattern()?];
                while self.peek_token()? == &Token::Comma {
                    self.consume_token()?;
                    patterns.push(self.parse_pattern()?);
                }
                self.expect_token(Token::RParen)?;
                if patterns.len() == 1 {
                    Ok(patterns.remove(0))
                } else {
                    Ok(Pattern::PTuple(patterns))
                }
            }
            Token::LBracket => {
                self.consume_token()?;
                let mut patterns = Vec::new();
                if self.peek_token()? != &Token::RBracket {
                    patterns.push(self.parse_pattern()?);
                    while self.peek_token()? == &Token::Comma {
                        self.consume_token()?;
                        patterns.push(self.parse_pattern()?);
                    }
                }
                self.expect_token(Token::RBracket)?;
                Ok(Pattern::PList(patterns))
            }
            Token::Underscore => {
                self.consume_token()?;
                Ok(Pattern::PWildcard)
            }
            Token::ConIdent(_) => {
                let con = self.expect_con_ident()?;
                let mut args = Vec::new();
                while let Ok(pat) = self.parse_pattern_atom() {
                    args.push(pat);
                }
                Ok(Pattern::PCon(con, args))
            }
            Token::Ident(_) => Ok(Pattern::PVar(self.expect_ident()?)),
            Token::Int(_) | Token::Float(_) | Token::Char(_) | Token::String(_) | Token::Ident(_) => {
                Ok(Pattern::PLit(self.parse_literal()?))
            }
            t => Err(ParseError::UnexpectedToken(format!("Expected pattern, got {:?}", t))),
        }
    }

    fn parse_pattern(&mut self) -> Result<Pattern, ParseError> {
        self.parse_pattern_atom()
    }

    fn parse_expr_atom(&mut self) -> Result<Expr, ParseError> {
        match self.peek_token()? {
            Token::LParen => {
                self.consume_token()?;
                let mut exprs = vec![self.parse_expr()?];
                while self.peek_token()? == &Token::Comma {
                    self.consume_token()?;
                    exprs.push(self.parse_expr()?);
                }
                self.expect_token(Token::RParen)?;
                if exprs.len() == 1 {
                    Ok(exprs.remove(0))
                } else {
                    Ok(Expr::Tuple(exprs))
                }
            }
            Token::LBracket => {
                self.consume_token()?;
                let mut exprs = Vec::new();
                if self.peek_token()? != &Token::RBracket {
                    exprs.push(self.parse_expr()?);
                    while self.peek_token()? == &Token::Comma {
                        self.consume_token()?;
                        exprs.push(self.parse_expr()?);
                    }
                }
                self.expect_token(Token::RBracket)?;
                Ok(Expr::List(exprs))
            }
            Token::Backslash => {
                self.consume_token()?;
                let mut params = vec![self.parse_pattern()?];
                while let Ok(pat) = self.parse_pattern() {
                    params.push(pat);
                }
                self.expect_token(Token::Arrow)?;
                let body = self.parse_expr()?;
                Ok(Expr::Lambda(params, Box::new(body)))
            }
            Token::Keyword(kw) if kw == "if" => {
                self.consume_token()?;
                let cond = self.parse_expr()?;
                self.expect_keyword("then")?;
                let then_expr = self.parse_expr()?;
                self.expect_keyword("else")?;
                let else_expr = self.parse_expr()?;
                Ok(Expr::If(Box::new(cond), Box::new(then_expr), Box::new(else_expr)))
            }
            Token::Keyword(kw) if kw == "let" => {
                self.consume_token()?;
                let mut decls = Vec::new();
                while self.peek_token()? != &Token::Keyword("in".to_string()) {
                    decls.push(self.parse_function_decl()?);
                }
                self.expect_keyword("in")?;
                let body = self.parse_expr()?;
                Ok(Expr::Let(decls, Box::new(body)))
            }
            Token::Keyword(kw) if kw == "case" => {
                self.consume_token()?;
                let expr = self.parse_expr()?;
                self.expect_keyword("of")?;
                let mut alts = Vec::new();
                while self.peek_token()? != &Token::Keyword("where".to_string()) 
                    && self.peek_token()? != &Token::EOF {
                    let pat = self.parse_pattern()?;
                    self.expect_token(Token::Arrow)?;
                    let expr = self.parse_expr()?;
                    alts.push(Alt { pat, expr });
                }
                Ok(Expr::Case(Box::new(expr), alts))
            }
            Token::Ident(_) => Ok(Expr::Var(self.expect_ident()?)),
            Token::Int(_) | Token::Float(_) | Token::Char(_) | Token::String(_) | Token::Ident(_) => {
                Ok(Expr::Lit(self.parse_literal()?))
            }
            t => Err(ParseError::UnexpectedToken(format!("Expected expression, got {:?}", t))),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_expr_atom()?;
        
        // Handle application
        while let Ok(right) = self.parse_expr_atom() {
            left = Expr::App(Box::new(left), Box::new(right));
        }
        
        Ok(left)
    }

    fn parse_import_spec(&mut self) -> Result<ImportSpec, ParseError> {
        match self.peek_token()? {
            Token::ConIdent(_) => Ok(ImportSpec::Type(self.expect_con_ident()?)),
            Token::Ident(_) => Ok(ImportSpec::Function(self.expect_ident()?)),
            Token::Symbol(s) if s == ".." => {
                self.consume_token()?;
                Ok(ImportSpec::All)
            }
            t => Err(ParseError::UnexpectedToken(format!("Expected import spec, got {:?}", t))),
        }
    }

    fn parse_import(&mut self) -> Result<Import, ParseError> {
        self.expect_keyword("import")?;
        
        let qualified = if self.peek_token()? == &Token::Keyword("qualified".to_string()) {
            self.consume_token()?;
            true
        } else {
            false
        };
        
        let module = {
            let mut parts = vec![self.expect_con_ident()?];
            while self.peek_token()? == &Token::Dot {
                self.consume_token()?;
                parts.push(self.expect_con_ident()?);
            }
            parts.join(".")
        };
        
        let as_name = if self.peek_token()? == &Token::Keyword("as".to_string()) {
            self.consume_token()?;
            Some({
                let mut parts = vec![self.expect_con_ident()?];
                while self.peek_token()? == &Token::Dot {
                    self.consume_token()?;
                    parts.push(self.expect_con_ident()?);
                }
                parts.join(".")
            })
        } else {
            None
        };
        
        let imports = if self.peek_token()? == &Token::LParen {
            self.consume_token()?;
            let mut specs = Vec::new();
            if self.peek_token()? != &Token::RParen {
                specs.push(self.parse_import_spec()?);
                while self.peek_token()? == &Token::Comma {
                    self.consume_token()?;
                    specs.push(self.parse_import_spec()?);
                }
            }
            self.expect_token(Token::RParen)?;
            Some(specs)
        } else {
            None
        };
        
        Ok(Import {
            module,
            qualified,
            as_name,
            imports,
        })
    }

    fn parse_data_decl(&mut self) -> Result<DataDecl, ParseError> {
        self.expect_keyword("data")?;
        
        let name = self.expect_con_ident()?;
        let mut type_vars = Vec::new();
        
        while let Token::Ident(_) = self.peek_token()? {
            type_vars.push(self.expect_ident()?);
        }
        
        self.expect_token(Token::Equals)?;
        
        let mut constructors = Vec::new();
        constructors.push(self.parse_constructor()?);
        
        while self.peek_token()? == &Token::Symbol("|".to_string()) {
            self.consume_token()?;
            constructors.push(self.parse_constructor()?);
        }
        
        Ok(DataDecl {
            name,
            type_vars,
            constructors,
        })
    }

    fn parse_constructor(&mut self) -> Result<Constructor, ParseError> {
        let name = self.expect_con_ident()?;
        let mut args = Vec::new();
        
        while let Ok(typ) = self.parse_type_atom() {
            args.push(typ);
        }
        
        Ok(Constructor { name, args })
    }

    fn parse_type_sig(&mut self) -> Result<(String, Type), ParseError> {
        let name = self.expect_ident()?;
        self.expect_token(Token::DoubleColon)?;
        let typ = self.parse_type()?;
        Ok((name, typ))
    }

    fn parse_equation(&mut self) -> Result<Equation, ParseError> {
        let lhs = self.parse_pattern()?;
        self.expect_token(Token::Equals)?;
        let rhs = self.parse_expr()?;
        
        let where_clause = if self.peek_token()? == &Token::Keyword("where".to_string()) {
            self.consume_token()?;
            let mut decls = Vec::new();
            while self.peek_token()? != &Token::EOF && self.peek_token()? != &Token::Keyword("module".to_string()) {
                decls.push(self.parse_function_decl()?);
            }
            Some(decls)
        } else {
            None
        };
        
        Ok(Equation {
            lhs,
            rhs,
            where_clause,
        })
    }

    fn parse_function_decl(&mut self) -> Result<FunctionDecl, ParseError> {
        let type_sig = if let Ok((name, typ)) = self.parse_type_sig() {
            Some((vec![name.clone()], typ))
        } else {
            None
        };
        
        let name = if let Some((ref names, _)) = type_sig {
            names[0].clone()
        } else {
            // Get name from first equation
            match self.peek_token()? {
                Token::Ident(_) => self.expect_ident()?,
                Token::ConIdent(_) => self.expect_con_ident()?,
                _ => return Err(ParseError::UnexpectedToken("Expected function name".to_string())),
            }
        };
        
        let mut equations = Vec::new();
        equations.push(self.parse_equation()?);
        
        while let Ok(eq) = self.parse_equation() {
            equations.push(eq);
        }
        
        Ok(FunctionDecl {
            name,
            type_sig,
            equations,
        })
    }

    fn parse_decl(&mut self) -> Result<Decl, ParseError> {
        match self.peek_token()? {
            Token::Keyword(kw) if kw == "data" => Ok(Decl::Data(self.parse_data_decl()?)),
            Token::Keyword(kw) if kw == "import" => Err(ParseError::Custom("Imports must be at the top of the module".to_string())),
            _ => {
                if let Ok((name, typ)) = self.parse_type_sig() {
                    Ok(Decl::TypeSig(name, typ))
                } else {
                    Ok(Decl::Function(self.parse_function_decl()?))
                }
            }
        }
    }

    pub fn parse_module(&mut self) -> Result<Module, ParseError> {
        // Skip shebang if present
        if self.peek_token()? == &Token::Symbol("#!".to_string()) {
            while self.peek_token()? != &Token::EOF && self.next_char() != Some('\n') {}
        }
        
        self.expect_keyword("module")?;
        let name = {
            let mut parts = vec![self.expect_con_ident()?];
            while self.peek_token()? == &Token::Dot {
                self.consume_token()?;
                parts.push(self.expect_con_ident()?);
            }
            parts.join(".")
        };
        self.expect_keyword("where")?;
        
        let mut imports = Vec::new();
        while self.peek_token()? == &Token::Keyword("import".to_string()) {
            imports.push(self.parse_import()?);
        }
        
        let mut decls = Vec::new();
        while self.peek_token()? != &Token::EOF {
            decls.push(self.parse_decl()?);
        }
        
        Ok(Module {
            name,
            imports,
            decls,
        })
    }
}

pub fn parse_haskell(source: &str) -> Result<Module, ParseError> {
    let mut parser = Parser::new(source);
    parser.parse_module()
}