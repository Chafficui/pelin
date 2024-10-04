#[derive(Debug, PartialEq, Clone)]
pub enum Token {
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
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }

        tokens.push(Token::EOF);
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>, String> {
        self.skip_whitespace();

        if self.is_at_end() {
            return Ok(None);
        }

        let c = self.peek().unwrap();
        match c {
            '(' => { self.advance(); Ok(Some(Token::LeftParen)) },
            ')' => { self.advance(); Ok(Some(Token::RightParen)) },
            '{' => { self.advance(); Ok(Some(Token::LeftBrace)) },
            '}' => { self.advance(); Ok(Some(Token::RightBrace)) },
            ',' => { self.advance(); Ok(Some(Token::Comma)) },
            '"' => self.string(),
            '=' => Ok(Some(Token::Equal)),
            '.' => { self.advance(); Ok(Some(Token::Dot)) },
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier_or_keyword(),
            _ => Err(format!("Unexpected character: {}", c)),
        }
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
        c
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }

    fn string(&mut self) -> Result<Option<Token>, String> {
        self.advance(); // Consume the opening quote
        let mut value = String::new();
        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance(); // Consume the closing quote
                return Ok(Some(Token::String(value)));
            }
            value.push(self.advance());
        }
        Err("Unterminated string".to_string())
    }

    fn number(&mut self) -> Result<Option<Token>, String> {
        let mut value = String::new();
        let mut has_decimal = false;

        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                value.push(self.advance());
            } else if c == '.' {
                if has_decimal {
                    return Err("Invalid number format: multiple decimal points".to_string());
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
            return Ok(Some(Token::Identifier(value)));
        }

        value.parse::<f64>()
            .map(|n| Some(Token::Number(n)))
            .map_err(|_| "Invalid number format".to_string())
    }

    fn identifier_or_keyword(&mut self) -> Result<Option<Token>, String> {
        let mut value = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                value.push(self.advance());
            } else {
                break;
            }
        }
        Ok(Some(match value.as_str() {
            "fn" => Token::Fn,
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            "nun" => Token::Nun,
            "return" => Token::Return,
            "imp" => Token::Imp,
            _ => Token::Identifier(value),
        }))
    }
}