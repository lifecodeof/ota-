use std::collections::HashMap;
use crate::types::{VariableType, VariableValue};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Variable {
    pub name: String,
    pub var_type: VariableType,
    pub value: Option<VariableValue>,
}

#[allow(dead_code)]
pub struct SymbolTable {
    scopes: Vec<HashMap<String, Variable>>,
}

#[allow(dead_code)]
impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn insert(&mut self, name: String, var_type: VariableType) {
        let variable = Variable {
            name: name.clone(),
            var_type,
            value: None,
        };
        self.scopes.last_mut().unwrap().insert(name, variable);
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
