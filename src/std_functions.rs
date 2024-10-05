use crate::interpreter::Value;

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