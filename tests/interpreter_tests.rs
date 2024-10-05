use std::cell::RefCell;
use std::rc::Rc;
use pelin::feather::FeatherManager;
use pelin::lexer::{Lexer};
use pelin::parser::Parser;
use pelin::interpreter::{InterpretResult, Interpreter, Value};

fn interpret(input: &str) -> Result<Value, String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let expressions = parser.parse()?;

    let feather_manager = Rc::new(RefCell::new(FeatherManager::new(std::env::current_dir().unwrap())));
    let interpreter = Interpreter::new(Rc::clone(&feather_manager));

    let mut result = Value::Nun;
    for expr in expressions {
        match interpreter.interpret(&expr)? {
            InterpretResult::Value(value) => result = value,
            InterpretResult::Return(value) => return Ok(value),
        }
    }
    Ok(result)
}

#[test]
fn test_interpret_number() {
    assert_eq!(interpret("42"), Ok(Value::Number(42.0)));
}

#[test]
fn test_interpret_string() {
    assert_eq!(interpret("\"Hello, Pelikan!\""), Ok(Value::String("Hello, Pelikan!".to_string())));
}

#[test]
fn test_interpret_boolean() {
    assert_eq!(interpret("true"), Ok(Value::Boolean(true)));
    assert_eq!(interpret("false"), Ok(Value::Boolean(false)));
}

#[test]
fn test_interpret_nun() {
    assert_eq!(interpret("nun"), Ok(Value::Nun));
}

#[test]
fn test_interpret_function_definition_and_call() {
    let input = r#"
        fn num identity(num x) { return x }
        identity(5)
    "#;
    assert_eq!(interpret(input), Ok(Value::Number(5.0)));
}

#[test]
fn test_interpret_nested_function_calls() {
    let input = r#"
        fn num add_one(num x) { return x }
        fn num multiply_by_two(num x) { return x }
        multiply_by_two(add_one(3))
    "#;
    assert_eq!(interpret(input), Ok(Value::Number(3.0)));
}

#[test]
fn test_interpret_multiple_statements() {
    let input = r#"
        fn nun set_global(num x) { return x }
        set_global(10)
        set_global(20)
    "#;
    assert_eq!(interpret(input), Ok(Value::Number(20.0)));
}

#[test]
fn test_interpret_function_with_multiple_parameters() {
    let input = r#"
        fn num sum_three(num a, num b, num c) { return a }
        sum_three(1, 2, 3)
    "#;
    assert_eq!(interpret(input), Ok(Value::Number(1.0)));
}

#[test]
fn test_interpret_empty_function() {
    let input = r#"
        fn nun do_nothing() { }
        do_nothing()
    "#;
    assert_eq!(interpret(input), Ok(Value::Nun));
}

#[test]
fn test_interpret_early_return() {
    let input = r#"
        fn num early_return(num x) {
            return x
            return 100
        }
        early_return(5)
    "#;
    assert_eq!(interpret(input), Ok(Value::Number(5.0)));
}

#[test]
fn test_interpret_undefined_variable() {
    let input = "undefined_var";
    assert!(interpret(input).is_err());
}

#[test]
fn test_interpret_invalid_function_call() {
    let input = "42()";
    assert!(interpret(input).is_err());
}

#[test]
fn test_interpret_import() {
    let input = "imp std_math";
    assert_eq!(interpret(input), Ok(Value::Nun));
}

#[test]
fn test_interpret_rust_function_call() {
    let input = "RUST[std_func::add](5, 3)";
    assert_eq!(interpret(input), Ok(Value::Number(8.0)));
}

#[test]
fn test_interpret_feather_function() {
    let input = r#"
        imp std_math
        std_math.add(5, 3)
    "#;
    assert_eq!(interpret(input), Ok(Value::Number(8.0)));
}