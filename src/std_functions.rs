use std::fs::File;
use std::io::{Read, Write};
use crate::interpreter::Value;

// num
pub fn std_num_add(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("add function expects 2 arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
        _ => Err("add function expects number arguments".to_string()),
    }
}

pub fn std_num_subtract(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("subtract function expects 2 arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
        _ => Err("subtract function expects number arguments".to_string()),
    }
}

pub fn std_num_multiply(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("multiply function expects 2 arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
        _ => Err("multiply function expects number arguments".to_string()),
    }
}

pub fn std_num_divide(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("divide function expects 2 arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => {
            if *b == 0.0 {
                Err("division by zero".to_string())
            } else {
                Ok(Value::Number(a / b))
            }
        }
        _ => Err("divide function expects number arguments".to_string()),
    }
}

pub fn std_num_sqrt(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("sqrt function expects 1 argument".to_string());
    }
    match &args[0] {
        Value::Number(a) => {
            if *a < 0.0 {
                Err("cannot compute square root of negative number".to_string())
            } else {
                Ok(Value::Number(a.sqrt()))
            }
        }
        _ => Err("sqrt function expects a number argument".to_string()),
    }
}

// math
pub fn std_math_sin(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("sin function expects 1 argument".to_string());
    }
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.sin())),
        _ => Err("sin function expects a number argument".to_string()),
    }
}

pub fn std_math_cos(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("cos function expects 1 argument".to_string());
    }
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.cos())),
        _ => Err("cos function expects a number argument".to_string()),
    }
}

// conversion
pub fn std_convert_to_string(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("to_string function expects 1 argument".to_string());
    }
    match &args[0] {
        Value::Number(n) => Ok(Value::String(n.to_string())),
        Value::Boolean(b) => Ok(Value::String(b.to_string())),
        Value::String(s) => Ok(Value::String(s.clone())),
        Value::Nun => Ok(Value::String("nun".to_string())),
        _ => Err("to_string function cannot convert this type".to_string()),
    }
}

pub fn std_convert_to_number(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("to_number function expects 1 argument".to_string());
    }
    match &args[0] {
        Value::String(s) => {
            s.parse::<f64>()
                .map(Value::Number)
                .map_err(|_| "Failed to convert string to number".to_string())
        }
        Value::Number(n) => Ok(Value::Number(*n)),
        _ => Err("to_number function can only convert strings or numbers".to_string()),
    }
}

// io
pub fn std_io_print(args: Vec<Value>) -> Result<Value, String> {
    for arg in args {
        print!("{:?}", arg);
    }
    Ok(Value::Nun)
}

// if, while, for, etc.
pub fn std_control_if(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("if function expects 3 arguments".to_string());
    }
    match &args[0] {
        Value::Boolean(b) => {
            if *b {
                Ok(args[1].clone())
            } else {
                Ok(args[2].clone())
            }
        }
        _ => Err("if function expects a boolean argument".to_string()),
    }
}

// file
pub fn std_file_read(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("file_read function expects 1 argument".to_string());
    }
    match &args[0] {
        Value::String(filename) => {
            let mut file = File::open(filename)
                .map_err(|e| format!("Failed to open file: {}", e))?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|e| format!("Failed to read file: {}", e))?;
            Ok(Value::String(contents))
        }
        _ => Err("file_read function expects a string argument".to_string()),
    }
}

pub fn std_file_write(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("file_write function expects 2 arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (Value::String(filename), Value::String(contents)) => {
            let mut file = File::create(filename)
                .map_err(|e| format!("Failed to create file: {}", e))?;
            file.write_all(contents.as_bytes())
                .map_err(|e| format!("Failed to write to file: {}", e))?;
            Ok(Value::Nun)
        }
        _ => Err("file_write function expects two string arguments".to_string()),
    }
}