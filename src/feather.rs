use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use std::sync::{Arc, Mutex};
use libloading::{Library, Symbol};
use log::{debug, error, info, trace};
use crate::interpreter::Value;
use crate::lexer::Lexer;
use crate::parser::{Parser, Expr};
use crate::std_functions::*;

pub type FeatherFunction = Arc<dyn Fn(Vec<Value>) -> Result<Value, String> + Send + Sync>;

pub struct Feather {
    pub name: String,
    pub functions: HashMap<String, FeatherFunction>,
}

impl Clone for Feather {
    fn clone(&self) -> Self {
        Feather {
            name: self.name.clone(),
            functions: self.functions.clone(),
        }
    }
}

pub struct FeatherManager {
    pub feathers: HashMap<String, Feather>,
    pub project_root: PathBuf,
    pub libraries: Mutex<HashMap<String, Arc<Library>>>,
    pub std_functions: HashMap<String, FeatherFunction>,
}

impl FeatherManager {
    pub fn new(project_root: PathBuf) -> Self {
        info!("Creating new FeatherManager with project root: {:?}", project_root);
        let mut manager = FeatherManager {
            feathers: HashMap::new(),
            project_root,
            libraries: Mutex::new(HashMap::new()),
            std_functions: HashMap::new(),
        };
        manager.register_std_functions();
        manager
    }

    fn register_std_functions(&mut self) {
        debug!("Registering standard functions");
        self.std_functions.insert("add".to_string(), Arc::new(std_num_add));
        self.std_functions.insert("subtract".to_string(), Arc::new(std_num_subtract));
        self.std_functions.insert("multiply".to_string(), Arc::new(std_num_multiply));
        self.std_functions.insert("divide".to_string(), Arc::new(std_num_divide));
        self.std_functions.insert("sqrt".to_string(), Arc::new(std_num_sqrt));
        debug!("Standard functions registered: {:?}", self.std_functions.keys());
    }

    pub fn import(&mut self, name: &str) -> Result<(), String> {
        info!("Attempting to import feather: {}", name);
        let path = if name.starts_with('.') {
            self.project_root.join(name.trim_start_matches('.'))
        } else {
            self.project_root.join("feathers").join(name)
        };

        let path = path.with_extension("pl");
        debug!("Full path for feather: {:?}", path);

        if !path.exists() {
            error!("Feather file not found: {:?}", path);
            return Err(format!("Could not find Feather file: {}", path.display()));
        }

        let content = fs::read_to_string(&path)
            .map_err(|e| {
                error!("Failed to read feather file: {:?}. Error: {:?}", path, e);
                format!("Failed to read file: {:?}", e)
            })?;

        debug!("Feather file content:\n{}", content);

        let mut lexer = Lexer::new(&content);
        let tokens = lexer.tokenize()?;
        debug!("Tokenization successful. Token count: {}", tokens.len());

        let mut parser = Parser::new(tokens);
        let expressions = parser.parse()?;
        debug!("Parsing successful. Expression count: {}", expressions.len());

        let mut feather = Feather {
            name: name.to_string(),
            functions: HashMap::new(),
        };

        let feather_manager = Arc::new(self.clone());

        for expr in expressions {
            if let Expr::FunctionDefinition { name, parameters: _, body, .. } = expr {
                debug!("Processing function definition: {}", name);
                let feather_manager = Arc::clone(&feather_manager);
                let func_name = name.clone(); // Clone the name for use in the closure
                let func = Arc::new(move |args: Vec<Value>| -> Result<Value, String> {
                    trace!("Calling feather function: {} with args: {:?}", func_name, args);
                    if let Some(body_expr) = body.first() {
                        if let Expr::RustFunctionCall { path, arguments: _ } = &**body_expr {
                            feather_manager.call_rust_function(&path.join("::"), args)
                        } else {
                            error!("Function body does not contain a Rust function call");
                            Err("Function body does not contain a Rust function call".to_string())
                        }
                    } else {
                        error!("Function body is empty");
                        Err("Function body is empty".to_string())
                    }
                });
                feather.functions.insert(name.clone(), func);
                debug!("Function '{}' added to feather", name);
            }
        }

        self.feathers.insert(name.to_string(), feather);
        info!("Feather '{}' successfully imported", name);
        Ok(())
    }

    fn call_rust_function(&self, path: &str, args: Vec<Value>) -> Result<Value, String> {
        debug!("Calling Rust function: {} with args: {:?}", path, args);
        let parts: Vec<&str> = path.split("::").collect();
        if parts.len() < 2 {
            error!("Invalid Rust function path: {}", path);
            return Err(format!("Invalid Rust function path: {}", path));
        }

        let library_name = parts[0];
        let function_name = parts.last().unwrap();

        let library = match self.load_library(library_name) {
            Ok(lib) => lib,
            Err(e) => {
                error!("Failed to load library '{}': {}", library_name, e);
                return Err(format!("Failed to load library '{}': {}", library_name, e));
            }
        };

        unsafe {
            let func: Symbol<unsafe fn(*const Value, usize) -> *mut Value> = match library.get(function_name.as_bytes()) {
                Ok(f) => f,
                Err(e) => {
                    error!("Failed to load function '{}' from library '{}': {:?}", function_name, library_name, e);
                    return Err(format!("Failed to load function '{}' from library '{}': {:?}", function_name, library_name, e));
                }
            };

            trace!("Calling Rust function");
            let result_ptr = func(args.as_ptr(), args.len());
            if result_ptr.is_null() {
                error!("Rust function '{}' returned null", function_name);
                Err(format!("Rust function '{}' returned null", function_name))
            } else {
                let result = Box::from_raw(result_ptr);
                debug!("Rust function call successful. Result: {:?}", result);
                Ok(*result)
            }
        }
    }

    pub fn call_function(&self, feather_name: &str, function_name: &str, arguments: Vec<Value>) -> Result<Value, String> {
        debug!("Calling function '{}' from feather '{}' with args: {:?}", function_name, feather_name, arguments);
        if feather_name == "std_func" {
            if let Some(func) = self.std_functions.get(function_name) {
                debug!("Calling standard function: {}", function_name);
                return func(arguments);
            }
        }

        let feather = self.feathers.get(feather_name)
            .ok_or_else(|| {
                error!("Feather '{}' not found", feather_name);
                format!("Feather '{}' not found", feather_name)
            })?;

        let function = feather.functions.get(function_name)
            .ok_or_else(|| {
                error!("Function '{}' not found in feather '{}'", function_name, feather_name);
                format!("Function '{}' not found in feather '{}'", function_name, feather_name)
            })?;

        debug!("Calling feather function");
        let result = function(arguments);
        debug!("Feather function call result: {:?}", result);
        result
    }

    fn load_library(&self, name: &str) -> Result<Arc<Library>, String> {
        debug!("Attempting to load library: {}", name);
        let mut libraries = self.libraries.lock().unwrap();
        if let Some(lib) = libraries.get(name) {
            debug!("Library '{}' already loaded", name);
            Ok(lib.clone())
        } else {
            let path = self.project_root.join("rust_libs").join(format!("lib{}.so", name));
            debug!("Loading library from path: {:?}", path);
            let library = Arc::new(unsafe {
                Library::new(&path).map_err(|e| {
                    error!("Failed to load library '{}' from {:?}: {:?}", name, path, e);
                    format!("Failed to load library: {:?}", e)
                })?
            });
            libraries.insert(name.to_string(), library.clone());
            info!("Library '{}' successfully loaded", name);
            Ok(library)
        }
    }
}

impl Clone for FeatherManager {
    fn clone(&self) -> Self {
        FeatherManager {
            feathers: self.feathers.clone(),
            project_root: self.project_root.clone(),
            libraries: Mutex::new(self.libraries.lock().unwrap().clone()),
            std_functions: self.std_functions.clone(),
        }
    }
}