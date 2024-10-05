use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use log::trace;
use crate::feather::FeatherManager;
use crate::parser::{Expr, Type};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nun,
    Function(Rc<Function>),
    FeatherFunction(String, String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterpretResult {
    Value(Value),
    Return(Value),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    params: Vec<(Type, String)>,
    body: Vec<Rc<Expr>>,
    closure: Rc<RefCell<Environment>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.values.get(name).cloned().or_else(|| {
            self.enclosing.as_ref().and_then(|env| env.borrow().get(name))
        })
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(format!("Undefined variable '{}'.", name))
        }
    }
}

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
    feather_manager: Rc<RefCell<FeatherManager>>,
}

impl Interpreter {
    pub fn new(feather_manager: Rc<RefCell<FeatherManager>>) -> Self {
        Interpreter {
            environment: Rc::new(RefCell::new(Environment::new())),
            feather_manager,
        }
    }

    pub fn interpret_program(&self, expressions: &[Expr]) -> Result<Value, String> {
        let mut result = Value::Nun;
        for expr in expressions {
            match self.interpret(expr)? {
                InterpretResult::Value(value) => result = value,
                InterpretResult::Return(value) => return Ok(value),
            }
        }
        Ok(result)
    }

    pub fn interpret(&self, expr: &Expr) -> Result<InterpretResult, String> {
        match expr {
            Expr::Number(n) => {
                trace!("Interpreting number: {}", n);
                Ok(InterpretResult::Value(Value::Number(*n)))
            },
            Expr::String(s) => {
                trace!("Interpreting string: {}", s);
                Ok(InterpretResult::Value(Value::String(s.clone())))
            },
            Expr::Boolean(b) => {
                trace!("Interpreting boolean: {}", b);
                Ok(InterpretResult::Value(Value::Boolean(*b)))
            },
            Expr::Nun => {
                trace!("Interpreting nun");
                Ok(InterpretResult::Value(Value::Nun))
            },
            Expr::Identifier(name) => {
                trace!("Interpreting identifier: {}", name);
                self.environment.borrow().get(name)
                    .map(InterpretResult::Value)
                    .ok_or_else(|| format!("Undefined variable '{}'.", name))
            },
            Expr::Return(value) => {
                trace!("Interpreting return");
                let value = self.interpret(value)?;
                match value {
                    InterpretResult::Value(v) => Ok(InterpretResult::Return(v)),
                    InterpretResult::Return(v) => Ok(InterpretResult::Return(v)),
                }
            },
            Expr::Assignment { name, value } => {
                let value = self.interpret(value)?;
                match value {
                    InterpretResult::Value(v) => {
                        self.environment.borrow_mut().assign(&name, v.clone())?;
                        Ok(InterpretResult::Value(v))
                    },
                    InterpretResult::Return(_) => Err("Cannot assign a return value".to_string()),
                }
            },
            Expr::FunctionDefinition { return_type: _, name, parameters, body } => {
                let function = Function {
                    params: parameters.clone(),
                    body: body.iter().map(|expr| Rc::new((**expr).clone())).collect(),
                    closure: Rc::clone(&self.environment),
                };
                self.environment.borrow_mut().define(name.clone(), Value::Function(Rc::new(function)));
                Ok(InterpretResult::Value(Value::Nun))
            },
            Expr::Import(name) => {
                trace!("Interpreting import: {}", name);
                self.feather_manager.borrow_mut().import(name)?;
                Ok(InterpretResult::Value(Value::Nun))
            },
            Expr::RustFunctionCall { path, arguments } => {
                trace!("Interpreting Rust function call: {:?}", path);
                let mut arg_values = Vec::new();
                for arg in arguments {
                    match self.interpret(arg)? {
                        InterpretResult::Value(v) => arg_values.push(v),
                        InterpretResult::Return(_) => return Err("Unexpected return".to_string()),
                    }
                }
                self.call_rust_function(path, arg_values)
            },
            Expr::FunctionCall { callee, arguments } => {
                let callee_value = self.interpret(callee)?;
                let mut arg_values = Vec::new();
                for arg in arguments {
                    match self.interpret(arg)? {
                        InterpretResult::Value(v) => arg_values.push(v),
                        InterpretResult::Return(_) => return Err("Unexpected return".to_string()),
                    }
                }
                self.call_function(callee_value, arg_values)
            },
            Expr::FeatherFunctionCall { feather, function, arguments } => {
                let mut arg_values = Vec::new();
                for arg in arguments {
                    match self.interpret(arg)? {
                        InterpretResult::Value(v) => arg_values.push(v),
                        InterpretResult::Return(_) => return Err("Unexpected return".to_string()),
                    }
                }
                let result = self.feather_manager.borrow().call_function(feather, function, arg_values)?;
                Ok(InterpretResult::Value(result))
            },
        }
    }

    fn call_rust_function(&self, path: &[String], arguments: Vec<Value>) -> Result<InterpretResult, String> {
        if path.len() < 2 {
            return Err("Invalid Rust function path".to_string());
        }
        let feather_name = &path[0];
        let function_name = path[1..].join("::");

        self.feather_manager.borrow().call_function(feather_name, &function_name, arguments)
            .map(InterpretResult::Value)
    }

    fn call_function(&self, callee: InterpretResult, arguments: Vec<Value>) -> Result<InterpretResult, String> {
        match callee {
            InterpretResult::Value(Value::Function(function)) => {
                let new_env = Rc::new(RefCell::new(Environment::new()));
                new_env.borrow_mut().enclosing = Some(Rc::clone(&function.closure));

                if function.params.len() != arguments.len() {
                    return Err(format!("Expected {} arguments but got {}.", function.params.len(), arguments.len()));
                }

                for ((_, param_name), arg_value) in function.params.iter().zip(arguments.iter()) {
                    new_env.borrow_mut().define(param_name.clone(), arg_value.clone());
                }

                let new_interpreter = Interpreter {
                    environment: new_env,
                    feather_manager: Rc::clone(&self.feather_manager),
                };

                let mut last_value = Value::Nun;
                for expr in &function.body {

                    match new_interpreter.interpret(expr)? {
                        InterpretResult::Return(value) => return Ok(InterpretResult::Value(value)),
                        InterpretResult::Value(value) => last_value = value,
                    }
                }

                Ok(InterpretResult::Value(last_value))
            },
            InterpretResult::Value(Value::FeatherFunction(feather_name, function_name)) => {
                self.feather_manager.borrow().call_function(&feather_name, &function_name, arguments)
                    .map(InterpretResult::Value)
            },
            _ => Err("Can only call functions.".to_string()),
        }
    }
}