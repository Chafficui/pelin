use std::collections::HashMap;
use std::path::{PathBuf};
use std::fs;
use crate::interpreter::Value;

#[derive(Debug, Clone)]
pub struct Feather {
    pub name: String,
    pub functions: HashMap<String, Value>,
}

pub struct FeatherManager {
    pub feathers: HashMap<String, Feather>,
    project_root: PathBuf,
}

impl FeatherManager {
    pub fn new(project_root: PathBuf) -> Self {
        FeatherManager {
            feathers: HashMap::new(),
            project_root,
        }
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
        
        // TODO implement feather import

        let feather = Feather {
            name: name.to_string(),
            functions: HashMap::new(),
        };

        self.feathers.insert(name.to_string(), feather);
        Ok(())
    }

    pub fn get_function(&self, feather_name: &str, function_name: &str) -> Option<&Value> {
        self.feathers.get(feather_name)?.functions.get(function_name)
    }
}