use crate::lexer::{Token, TokenType};

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Num,
    Str,
    Bool,
    Nun,
    Custom(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Boolean(bool),
    Nun,
    Return(Box<Expr>),
    Identifier(String),
    FunctionCall {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
    FunctionDefinition {
        return_type: Type,
        name: String,
        parameters: Vec<(Type, String)>,
        body: Vec<Box<Expr>>,
    },
    Assignment {
        name: String,
        value: Box<Expr>,
    },
    Import(String),
    RustFunctionCall {
        path: Vec<String>,
        arguments: Vec<Expr>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, String> {
        let mut expressions = Vec::new();
        while !self.is_at_end() {
            if self.match_token(&[TokenType::Imp]) {
                expressions.push(self.import_statement()?);
            } else {
                expressions.push(self.expression()?);
            }
        }
        Ok(expressions)
    }

    fn import_statement(&mut self) -> Result<Expr, String> {
        let name = self.consume_identifier("Expected feather name after 'imp'")?;
        Ok(Expr::Import(name))
    }

    fn expression(&mut self) -> Result<Expr, String> {
        if self.match_token(&[TokenType::RustKeyword]) {
            self.rust_function_call()
        } else if self.match_token(&[TokenType::Return]) {
            let value = self.expression()?;
            Ok(Expr::Return(Box::new(value)))
        } else if self.match_token(&[TokenType::Fn]) {
            self.function_definition()
        } else {
            self.function_call()
        }
    }

    fn rust_function_call(&mut self) -> Result<Expr, String> {
        self.consume(TokenType::LeftBracket, "Expected '[' after 'RUST'")?;
        let mut path = Vec::new();

        loop {
            path.push(self.consume_identifier("Expected identifier in Rust function path")?);
            if !self.match_token(&[TokenType::DoubleColon]) {
                break;
            }
        }

        self.consume(TokenType::RightBracket, "Expected ']' after Rust function path")?;

        self.consume(TokenType::LeftParen, "Expected '(' after Rust function path")?;
        let arguments = if self.check(&TokenType::RightParen) {
            Vec::new()
        } else {
            self.parse_arguments()?
        };
        self.consume(TokenType::RightParen, "Expected ')' after arguments in Rust function call")?;

        Ok(Expr::RustFunctionCall { path, arguments })
    }

    fn parse_arguments(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();
        loop {
            args.push(self.expression()?);
            if !self.match_token(&[TokenType::Comma]) {
                break;
            }
        }
        Ok(args)
    }

    fn function_definition(&mut self) -> Result<Expr, String> {
        let return_type = self.parse_type()?;
        let name = self.consume_identifier("Expected function name")?;
        self.consume(TokenType::LeftParen, "Expected '(' after function name")?;

        let mut parameters = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                let param_type = self.parse_type()?;
                let param_name = self.consume_identifier("Expected parameter name")?;
                parameters.push((param_type, param_name));
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;

        self.consume(TokenType::LeftBrace, "Expected '{' before function body")?;
        let mut body = Vec::new();
        while !self.check(&TokenType::RightBrace) {
            body.push(Box::new(self.expression()?));
        }
        self.consume(TokenType::RightBrace, "Expected '}' after function body")?;

        Ok(Expr::FunctionDefinition {
            return_type,
            name,
            parameters,
            body,
        })
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        if let Some(token) = self.advance() {
            match &token.kind {
                TokenType::Identifier(name) => Ok(match name.as_str() {
                    "num" => Type::Num,
                    "str" => Type::Str,
                    "bool" => Type::Bool,
                    "nun" => Type::Nun,
                    _ => Type::Custom(name.clone()),
                }),
                TokenType::Nun => Ok(Type::Nun),
                _ => Err(self.error_at_previous("Expected type name")),
            }
        } else {
            Err(self.error_at_end("Unexpected end of input while parsing type"))
        }
    }

    fn function_call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;

        if self.match_token(&[TokenType::LeftParen]) {
            let arguments = self.arguments()?;
            expr = Expr::FunctionCall {
                callee: Box::new(expr),
                arguments,
            };
        }

        Ok(expr)
    }

    fn arguments(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                args.push(self.expression()?);
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expected ')' after arguments")?;
        Ok(args)
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if let Some(token) = self.advance() {
            match &token.kind {
                TokenType::Number(n) => Ok(Expr::Number(*n)),
                TokenType::String(s) => Ok(Expr::String(s.clone())),
                TokenType::Boolean(b) => Ok(Expr::Boolean(*b)),
                TokenType::Nun => Ok(Expr::Nun),
                TokenType::Identifier(name) => Ok(Expr::Identifier(name.clone())),
                _ => Err(self.error_at_previous("Unexpected token")),
            }
        } else {
            Err(self.error_at_end("Unexpected end of input"))
        }
    }

    fn consume_identifier(&mut self, message: &str) -> Result<String, String> {
        if let Some(token) = self.advance() {
            match &token.kind {
                TokenType::Identifier(name) => Ok(name.clone()),
                _ => Err(self.error_at_previous(message)),
            }
        } else {
            Err(self.error_at_end(message))
        }
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, t: &TokenType) -> bool {
        self.peek().map_or(false, |token| &token.kind == t)
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().map_or(true, |t| matches!(t.kind, TokenType::EOF))
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn consume(&mut self, t: TokenType, message: &str) -> Result<&Token, String> {
        if self.check(&t) {
            Ok(self.advance().unwrap())
        } else {
            if self.is_at_end() {
                Err(self.error_at_end(message))
            } else {
                Err(self.error_at_current(message))
            }
        }
    }

    fn error_at_current(&self, message: &str) -> String {
        self.error_at(self.current, message)
    }

    fn error_at_previous(&self, message: &str) -> String {
        self.error_at(self.current - 1, message)
    }

    fn error_at_end(&self, message: &str) -> String {
        let last_token = self.tokens.last().unwrap();
        format!("[line {}, column {}] Error at end: {}",
                last_token.line, last_token.column, message)
    }

    fn error_at(&self, index: usize, message: &str) -> String {
        let token = &self.tokens[index];
        format!("[line {}, column {}] Error at '{}': {}",
                token.line, token.column, token.lexeme, message)
    }
}