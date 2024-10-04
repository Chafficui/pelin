use pelin::lexer::{tokens_to_token_types, Lexer};
use pelin::parser::{Parser, Expr, Type};

#[test]
fn test_parse_number() {
    let mut lexer = Lexer::new("42");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens_to_token_types(tokens));
    let expr = parser.parse().unwrap();
    assert_eq!(expr, vec![Expr::Number(42.0)]);
}

#[test]
fn test_parse_string() {
    let mut lexer = Lexer::new("\"Hello, Pelikan!\"");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens_to_token_types(tokens));
    let expr = parser.parse().unwrap();
    assert_eq!(expr, vec![Expr::String("Hello, Pelikan!".to_string())]);
}

#[test]
fn test_parse_identifier() {
    let mut lexer = Lexer::new("variable_name");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens_to_token_types(tokens));
    let expr = parser.parse().unwrap();
    assert_eq!(expr, vec![Expr::Identifier("variable_name".to_string())]);
}

#[test]
fn test_parse_function_call() {
    let mut lexer = Lexer::new("add(5, 3)");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens_to_token_types(tokens));
    let expr = parser.parse().unwrap();
    assert_eq!(expr, vec![Expr::FunctionCall {
        callee: Box::new(Expr::Identifier("add".to_string())),
        arguments: vec![Expr::Number(5.0), Expr::Number(3.0)],
    }]);
}

#[test]
fn test_parse_function_definition() {
    let mut lexer = Lexer::new("fn num add(num a, num b) { return a }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens_to_token_types(tokens));
    let expr = parser.parse().unwrap();
    assert_eq!(expr, vec![Expr::FunctionDefinition {
        return_type: Type::Num,
        name: "add".to_string(),
        parameters: vec![(Type::Num, "a".to_string()), (Type::Num, "b".to_string())],
        body: vec![Box::new(Expr::Return(Box::new(Expr::Identifier("a".to_string()))))],
    }]);
}

#[test]
fn test_parse_nested_function_calls() {
    let mut lexer = Lexer::new("outer(inner(42), another(true))");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens_to_token_types(tokens));
    let expr = parser.parse().unwrap();
    assert_eq!(expr, vec![Expr::FunctionCall {
        callee: Box::new(Expr::Identifier("outer".to_string())),
        arguments: vec![
            Expr::FunctionCall {
                callee: Box::new(Expr::Identifier("inner".to_string())),
                arguments: vec![Expr::Number(42.0)],
            },
            Expr::FunctionCall {
                callee: Box::new(Expr::Identifier("another".to_string())),
                arguments: vec![Expr::Boolean(true)],
            },
        ],
    }]);
}

#[test]
fn test_parse_return_statement() {
    let mut lexer = Lexer::new("return 42");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens_to_token_types(tokens));
    let expr = parser.parse().unwrap();
    assert_eq!(expr, vec![Expr::Return(Box::new(Expr::Number(42.0)))]);
}

#[test]
fn test_parse_function_with_multiple_statements() { //TODO implement assignments
    let mut lexer = Lexer::new("fn nun test() { x = 5 return x }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens_to_token_types(tokens));
    let expr = parser.parse().unwrap();
    assert_eq!(expr, vec![Expr::FunctionDefinition {
        return_type: Type::Nun,
        name: "test".to_string(),
        parameters: vec![],
        body: vec![
            Box::new(Expr::Assignment {
                name: "x".to_string(),
                value: Box::new(Expr::Number(5.0)),
            }),
            Box::new(Expr::Return(Box::new(Expr::Identifier("x".to_string())))),
        ],
    }]);
}

#[test]
fn test_parse_empty_function() {
    let mut lexer = Lexer::new("fn nun empty() { }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens_to_token_types(tokens));
    let expr = parser.parse().unwrap();
    assert_eq!(expr, vec![Expr::FunctionDefinition {
        return_type: Type::Nun,
        name: "empty".to_string(),
        parameters: vec![],
        body: vec![],
    }]);
}