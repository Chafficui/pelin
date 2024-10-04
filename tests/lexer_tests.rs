use pelin::lexer::{Lexer, TokenType, tokens_to_token_types};

#[test]
fn test_lexer_numbers() {
    let mut lexer = Lexer::new("42 3.14");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens_to_token_types(tokens), vec![
        TokenType::Number(42.0),
        TokenType::Number(3.14),
        TokenType::EOF
    ]);
}

#[test]
fn test_lexer_strings() {
    let mut lexer = Lexer::new("\"Hello, Pelikan!\"");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens_to_token_types(tokens), vec![
        TokenType::String("Hello, Pelikan!".to_string()),
        TokenType::EOF
    ]);
}

#[test]
fn test_lexer_identifiers_and_keywords() {
    let mut lexer = Lexer::new("fn return nun true false myVar if while");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens_to_token_types(tokens), vec![
        TokenType::Fn,
        TokenType::Return,
        TokenType::Nun,
        TokenType::Boolean(true),
        TokenType::Boolean(false),
        TokenType::Identifier("myVar".to_string()),
        TokenType::Identifier("if".to_string()),
        TokenType::Identifier("while".to_string()),
        TokenType::EOF
    ]);
}

#[test]
fn test_lexer_delimiters() {
    let mut lexer = Lexer::new("( ) { }, ");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens_to_token_types(tokens), vec![
        TokenType::LeftParen,
        TokenType::RightParen,
        TokenType::LeftBrace,
        TokenType::RightBrace,
        TokenType::Comma,
        TokenType::EOF
    ]);
}

#[test]
fn test_lexer_function_call() {
    let mut lexer = Lexer::new("add(5, 3)");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens_to_token_types(tokens), vec![
        TokenType::Identifier("add".to_string()),
        TokenType::LeftParen,
        TokenType::Number(5.0),
        TokenType::Comma,
        TokenType::Number(3.0),
        TokenType::RightParen,
        TokenType::EOF
    ]);
}

#[test]
fn test_lexer_empty_input() {
    let mut lexer = Lexer::new("");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens_to_token_types(tokens), vec![TokenType::EOF]);
}

#[test]
fn test_lexer_whitespace_only() {
    let mut lexer = Lexer::new("   \t\n   ");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens_to_token_types(tokens), vec![TokenType::EOF]);
}

#[test]
fn test_lexer_invalid_number() {
    let mut lexer = Lexer::new("42.42.42");
    assert!(lexer.tokenize().is_err());
}

#[test]
fn test_lexer_unterminated_string() {
    let mut lexer = Lexer::new("\"unterminated string");
    assert!(lexer.tokenize().is_err());
}

#[test]
fn test_lexer_complex_identifier() {
    let mut lexer = Lexer::new("_complex123_identifier");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens_to_token_types(tokens), vec![TokenType::Identifier("_complex123_identifier".to_string()), TokenType::EOF]);
}

#[test]
fn test_lexer_valid_number() {
    let mut lexer = Lexer::new("42.42");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens_to_token_types(tokens), vec![TokenType::Number(42.42), TokenType::EOF]);
}

#[test]
fn test_lexer_number_followed_by_dot() {
    let mut lexer = Lexer::new("42.add(3)");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens_to_token_types(tokens), vec![
        TokenType::Identifier(42.to_string()),
        TokenType::Dot,
        TokenType::Identifier("add".to_string()),
        TokenType::LeftParen,
        TokenType::Number(3.0),
        TokenType::RightParen,
        TokenType::EOF
    ]);
}

#[test]
fn test_lexer_feather_function_call() {
    let mut lexer = Lexer::new("std_math.add(5, 3)");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens_to_token_types(tokens), vec![
        TokenType::Identifier("std_math".to_string()),
        TokenType::Dot,
        TokenType::Identifier("add".to_string()),
        TokenType::LeftParen,
        TokenType::Number(5.0),
        TokenType::Comma,
        TokenType::Number(3.0),
        TokenType::RightParen,
        TokenType::EOF
    ]);
}