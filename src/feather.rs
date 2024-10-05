use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use std::sync::{Arc, Mutex};
use libloading::{Library, Symbol};
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
        self.std_functions.insert("add".to_string(), Arc::new(std_num_add));
        self.std_functions.insert("subtract".to_string(), Arc::new(std_num_subtract));
        self.std_functions.insert("multiply".to_string(), Arc::new(std_num_multiply));
        self.std_functions.insert("divide".to_string(), Arc::new(std_num_divide));
        self.std_functions.insert("sqrt".to_string(), Arc::new(std_num_sqrt));
    }

    pub fn import(&mut self, name: &str) -> Result<(), String> {
        let path = if name.starts_with('.') {
            self.project_root.join(name.trim_start_matches('.'))
        } else {
            self.project_root.join("feathers").join(name)
        };

        let path = path.with_extension("pl");

        if !path.exists() {
            return Err(format!("Could not find Feather file: {}", path.display()));
        }

        let content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read file: {:?}", e))?;

        let mut lexer = Lexer::new(&content);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let expressions = parser.parse()?;

        let mut feather = Feather {
            name: name.to_string(),
            functions: HashMap::new(),
        };

        let feather_manager = Arc::new(self.clone());

        for expr in expressions {
            if let Expr::FunctionDefinition { name, parameters: _, body, .. } = expr {
                let feather_manager = Arc::clone(&feather_manager);
                let func = Arc::new(move |args: Vec<Value>| -> Result<Value, String> {
                    if let Some(body_expr) = body.first() {
                        if let Expr::RustFunctionCall { path, arguments: _ } = &**body_expr {
                            feather_manager.call_rust_function(&path.join("::"), args)
                        } else {
                            Err("Function body does not contain a Rust function call".to_string())
                        }
                    } else {
                        Err("Function body is empty".to_string())
                    }
                });
                feather.functions.insert(name, func);
            }
        }

        self.feathers.insert(name.to_string(), feather);
        Ok(())
    }

    fn call_rust_function(&self, path: &str, args: Vec<Value>) -> Result<Value, String> {
        let parts: Vec<&str> = path.split("::").collect();
        if parts.len() < 2 {
            return Err("Invalid Rust function path".to_string());
        }

        let library_name = parts[0];
        let function_name = parts.last().unwrap();

        let library = self.load_library(library_name)?;

        unsafe {
            let func: Symbol<unsafe fn(*const Value, usize) -> *mut Value> =
                library.get(function_name.as_bytes())
                    .map_err(|e| format!("Failed to load function: {:?}", e))?;

            let result_ptr = func(args.as_ptr(), args.len());
            if result_ptr.is_null() {
                Err("Rust function returned null".to_string())
            } else {
                Ok(*Box::from_raw(result_ptr))
            }
        }
    }

    pub fn call_function(&self, feather_name: &str, function_name: &str, arguments: Vec<Value>) -> Result<Value, String> {
        if feather_name == "std_func" {
            if let Some(func) = self.std_functions.get(function_name) {
                return func(arguments);
            }
        }
        
        let feather = self.feathers.get(feather_name)
            .ok_or_else(|| format!("Feather '{}' not found", feather_name))?;

        let function = feather.functions.get(function_name)
            .ok_or_else(|| format!("Function '{}' not found in feather '{}'", function_name, feather_name))?;

        function(arguments)
    }

    fn load_library(&self, name: &str) -> Result<Arc<Library>, String> {
        let mut libraries = self.libraries.lock().unwrap();
        if let Some(lib) = libraries.get(name) {
            Ok(lib.clone())
        } else {
            let path = self.project_root.join("rust_libs").join(format!("lib{}.so", name));
            let library = Arc::new(unsafe {
                Library::new(path).map_err(|e| format!("Failed to load library: {:?}", e))?
            });
            libraries.insert(name.to_string(), library.clone());
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