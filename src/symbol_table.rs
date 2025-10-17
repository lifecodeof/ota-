use std::collections::HashMap;
use crate::types::{VariableType, VariableValue};

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub var_type: VariableType,
    pub value: Option<VariableValue>,
}

pub struct SymbolTable {
    symbols: HashMap<String, Variable>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, var_type: VariableType) {
        let variable = Variable {
            name: name.clone(),
            var_type,
            value: None,
        };
        self.symbols.insert(name, variable);
    }

    pub fn lookup(&self, name: &str) -> Option<&Variable> {
        self.symbols.get(name)
    }

    pub fn update_value(&mut self, name: &str, value: VariableValue) {
        if let Some(var) = self.symbols.get_mut(name) {
            var.value = Some(value);
        }
    }
}
