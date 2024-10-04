use crate::lexer::TokenType::Identifier;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Number(f64),
    String(String),
    Boolean(bool),
    Nun,
    Identifier(String),
    // Function related
    Fn,
    Return,
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Equal,
    // Import
    Imp,
    Dot,
    // End of input
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        loop {
            match self.next_token() {
                Ok(token) => {
                    if token.kind == TokenType::EOF {
                        tokens.push(token);
                        break;
                    } else {
                        tokens.push(token);
                    }
                }
                Err(err) => return Err(err),
            }
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        if self.is_at_end() {
            return Ok(Token { kind: TokenType::EOF, lexeme: "".to_string(), line: self.line, column: self.column });
        }

        let c = self.peek().unwrap();
        let start_column = self.column;

        let token = match c {
            '(' => { self.advance(); Ok(Token { kind: TokenType::LeftParen, lexeme: "(".to_string(), line: self.line, column: start_column }) },
            ')' => { self.advance(); Ok(Token { kind: TokenType::RightParen, lexeme: ")".to_string(), line: self.line, column: start_column }) },
            '{' => { self.advance(); Ok(Token { kind: TokenType::LeftBrace, lexeme: "{".to_string(), line: self.line, column: start_column }) },
            '}' => { self.advance(); Ok(Token { kind: TokenType::RightBrace, lexeme: "}".to_string(), line: self.line, column: start_column }) },
            ',' => { self.advance(); Ok(Token { kind: TokenType::Comma, lexeme: ",".to_string(), line: self.line, column: start_column }) },
            '"' => self.string(),
            '=' => { self.advance(); Ok(Token { kind: TokenType::Equal, lexeme: "=".to_string(), line: self.line, column: start_column }) },
            '.' => { self.advance(); Ok(Token { kind: TokenType::Dot, lexeme: ".".to_string(), line: self.line, column: start_column }) },
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier_or_keyword(),
            _ => Err(format!("Unexpected character: '{}' at line {}, column {}", c, self.line, self.column)),
        }?;

        Ok(token)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn advance(&mut self) -> char {
        let c = self.input[self.position];
        self.position += 1;

        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        c
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }

    fn string(&mut self) -> Result<Token, String> {
        self.advance();
        let mut value = String::new();
        let start_column = self.column;

        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance();
                return Ok(Token {
                    kind: TokenType::String(value.clone()),
                    lexeme: value,
                    line: self.line,
                    column: start_column,
                });
            }
            value.push(self.advance());
        }
        Err(format!("Unterminated string at line {}, column {}", self.line, self.column))
    }

    fn number(&mut self) -> Result<Token, String> {
        let mut value = String::new();
        let mut has_decimal = false;
        let start_column = self.column;

        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                value.push(self.advance());
            } else if c == '.' {
                if has_decimal {
                    return Err(format!("Invalid number format at line {}, column {}: multiple decimal points", self.line, self.column));
                } else if self.peek_next().map_or(false, |next| next.is_digit(10)) {
                    value.push(self.advance());
                    has_decimal = true;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        if self.peek() == Some('.') && !has_decimal {
            return Ok(Token {
                kind: Identifier(value.clone()),
                lexeme: value,
                line: self.line,
                column: start_column,
            })
        }

        value.parse::<f64>()
            .map(|n| Token {
                kind: TokenType::Number(n),
                lexeme: value.clone(),
                line: self.line,
                column: start_column,
            })
            .map_err(|_| format!("Invalid number format at line {}, column {}", self.line, self.column))
    }

    fn identifier_or_keyword(&mut self) -> Result<Token, String> {
        let mut value = String::new();
        let start_column = self.column;

        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                value.push(self.advance());
            } else {
                break;
            }
        }

        let kind = match value.as_str() {
            "fn" => TokenType::Fn,
            "true" => TokenType::Boolean(true),
            "false" => TokenType::Boolean(false),
            "nun" => TokenType::Nun,
            "return" => TokenType::Return,
            "imp" => TokenType::Imp,
            _ => Identifier(value.clone()),
        };

        Ok(Token {
            kind,
            lexeme: value,
            line: self.line,
            column: start_column,
        })
    }
}