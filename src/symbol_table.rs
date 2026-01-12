use crate::ast::{FunctionDefinition, StructDefinition};
use crate::types::{Type, VariableValue};
use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Variable {
    pub name: String,
    pub var_type: Type,
    pub value: Option<VariableValue>,
}

#[allow(dead_code)]
pub struct SymbolTable {
    scopes: Vec<HashMap<String, Variable>>,
    functions: HashMap<String, FunctionDefinition>,
    structs: HashMap<String, StructDefinition>,
}

#[allow(dead_code)]
impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
            structs: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, var_type: Type) {
        let variable = Variable {
            name: name.clone(),
            var_type,
            value: None,
        };
        self.scopes.last_mut().unwrap().insert(name, variable);
    }

    pub fn insert_function(&mut self, function: FunctionDefinition) -> Result<(), String> {
        if self.functions.contains_key(&function.name) {
            return Err(format!("Function '{}' already defined", function.name));
        }
        self.functions.insert(function.name.clone(), function);
        Ok(())
    }

    pub fn insert_struct(&mut self, struct_def: StructDefinition) -> Result<(), String> {
        if self.structs.contains_key(&struct_def.name) {
            return Err(format!("Struct '{}' already defined", struct_def.name));
        }
        self.structs.insert(struct_def.name.clone(), struct_def);
        Ok(())
    }

    pub fn lookup_function(&self, name: &str) -> Option<&FunctionDefinition> {
        self.functions.get(name)
    }

    pub fn lookup_struct(&self, name: &str) -> Option<&StructDefinition> {
        self.structs.get(name)
    }

    #[allow(dead_code)]
    pub fn lookup(&self, name: &str) -> Option<&Variable> {
        for scope in self.scopes.iter().rev() {
            if let Some(var) = scope.get(name) {
                return Some(var);
            }
        }
        None
    }

    #[allow(dead_code)]
    pub fn update_value(&mut self, name: &str, value: VariableValue) {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(var) = scope.get_mut(name) {
                var.value = Some(value);
                return;
            }
        }
    }

    #[allow(dead_code)]
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    #[allow(dead_code)]
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
