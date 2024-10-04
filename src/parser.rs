use crate::lexer::Token;

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
    }
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
            expressions.push(self.expression()?);
        }
        Ok(expressions)
    }

    fn expression(&mut self) -> Result<Expr, String> {
        if self.match_token(&[Token::Return]) {
            let value = self.expression()?;
            Ok(Expr::Return(Box::new(value)))
        } else {
            if self.match_token(&[Token::Fn]) {
                self.function_definition()
            } else {
                self.function_call()
            }
        }
    }

    fn function_definition(&mut self) -> Result<Expr, String> {
        let return_type = self.parse_type()?;
        let name = self.consume_identifier("Expect function name.")?;
        self.consume(Token::LeftParen, "Expect '(' after function name.")?;

        let mut parameters = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                let param_type = self.parse_type()?;
                let param_name = self.consume_identifier("Expect parameter name.")?;
                parameters.push((param_type, param_name));
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expect ')' after parameters.")?;

        self.consume(Token::LeftBrace, "Expect '{' before function body.")?;
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) {
            body.push(Box::new(self.expression()?));
        }
        self.consume(Token::RightBrace, "Expect '}' after function body.")?;

        Ok(Expr::FunctionDefinition {
            return_type,
            name,
            parameters,
            body,
        })
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        match self.advance() {
            Some(Token::Identifier(name)) => Ok(match name.as_str() {
                "num" => Type::Num,
                "str" => Type::Str,
                "bool" => Type::Bool,
                "nun" => Type::Nun,
                _ => Type::Custom(name.clone()),
            }),
            Some(Token::Nun) => Ok(Type::Nun),
            _ => Err("Expect type name.".to_string()),
        }
    }

    fn function_call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;

        if self.match_token(&[Token::LeftParen]) {
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

        if !self.check(&Token::RightParen) {
            loop {
                args.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }

        self.consume(Token::RightParen, "Expect ')' after arguments.")?;
        Ok(args)
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if let Some(token) = self.advance() {
            match token {
                Token::Number(n) => Ok(Expr::Number(*n)),
                Token::String(s) => Ok(Expr::String(s.clone())),
                Token::Boolean(b) => Ok(Expr::Boolean(*b)),
                Token::Nun => Ok(Expr::Nun),
                Token::Identifier(name) => Ok(Expr::Identifier(name.clone())),
                _ => Err(format!("Unexpected token: {:?}", token)),
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }

    fn consume_identifier(&mut self, message: &str) -> Result<String, String> {
        match self.advance() {
            Some(Token::Identifier(name)) => Ok(name.clone()),
            _ => Err(message.to_string()),
        }
    }

    fn match_token(&mut self, types: &[Token]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, t: &Token) -> bool {
        self.peek().map_or(false, |token| token == t)
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().map_or(true, |t| matches!(t, Token::EOF))
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        if self.current > 0 {
            self.tokens.get(self.current - 1)
        } else {
            None
        }
    }

    fn consume(&mut self, t: Token, message: &str) -> Result<&Token, String> {
        if self.check(&t) {
            Ok(self.advance().unwrap())
        } else {
            Err(message.to_string())
        }
    }
}
