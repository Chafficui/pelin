use pelin::lexer::{Lexer, Token};

#[test]
fn test_lexer_numbers() {
    let mut lexer = Lexer::new("42 3.14");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens, vec![
        Token::Number(42.0),
        Token::Number(3.14),
        Token::EOF
    ]);
}

#[test]
fn test_lexer_strings() {
    let mut lexer = Lexer::new("\"Hello, Pelikan!\"");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens, vec![
        Token::String("Hello, Pelikan!".to_string()),
        Token::EOF
    ]);
}

#[test]
fn test_lexer_identifiers_and_keywords() {
    let mut lexer = Lexer::new("fn return nun true false myVar if while");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens, vec![
        Token::Fn,
        Token::Return,
        Token::Nun,
        Token::Boolean(true),
        Token::Boolean(false),
        Token::Identifier("myVar".to_string()),
        Token::Identifier("if".to_string()),
        Token::Identifier("while".to_string()),
        Token::EOF
    ]);
}

#[test]
fn test_lexer_delimiters() {
    let mut lexer = Lexer::new("( ) { }, ");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens, vec![
        Token::LeftParen,
        Token::RightParen,
        Token::LeftBrace,
        Token::RightBrace,
        Token::Comma,
        Token::EOF
    ]);
}

#[test]
fn test_lexer_function_call() {
    let mut lexer = Lexer::new("add(5, 3)");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens, vec![
        Token::Identifier("add".to_string()),
        Token::LeftParen,
        Token::Number(5.0),
        Token::Comma,
        Token::Number(3.0),
        Token::RightParen,
        Token::EOF
    ]);
}

#[test]
fn test_lexer_empty_input() {
    let mut lexer = Lexer::new("");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens, vec![Token::EOF]);
}

#[test]
fn test_lexer_whitespace_only() {
    let mut lexer = Lexer::new("   \t\n   ");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens, vec![Token::EOF]);
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
    assert_eq!(tokens, vec![Token::Identifier("_complex123_identifier".to_string()), Token::EOF]);
}

#[test]
fn test_lexer_valid_number() {
    let mut lexer = Lexer::new("42.42");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens, vec![Token::Number(42.42), Token::EOF]);
}

#[test]
fn test_lexer_number_followed_by_dot() {
    let mut lexer = Lexer::new("42.add(3)");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens, vec![
        Token::Identifier(42.to_string()),
        Token::Dot,
        Token::Identifier("add".to_string()),
        Token::LeftParen,
        Token::Number(3.0),
        Token::RightParen,
        Token::EOF
    ]);
}

#[test]
fn test_lexer_feather_function_call() {
    let mut lexer = Lexer::new("std_math.add(5, 3)");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens, vec![
        Token::Identifier("std_math".to_string()),
        Token::Dot,
        Token::Identifier("add".to_string()),
        Token::LeftParen,
        Token::Number(5.0),
        Token::Comma,
        Token::Number(3.0),
        Token::RightParen,
        Token::EOF
    ]);
}