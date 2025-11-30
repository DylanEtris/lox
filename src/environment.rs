use std::collections::HashMap;

use crate::{
    error::RuntimeError,
    token::{Literal, Token},
};

#[derive(Default, Clone)]
pub struct Environment {
    values: HashMap<String, Literal>,
    pub enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(environment: Box<Environment>) -> Environment {
        Environment {
            values: HashMap::default(),
            enclosing: Some(environment),
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Literal, RuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(literal) => {
                return Ok(literal.clone());
            }
            None => {
                if let Some(environment) = &self.enclosing {
                    return environment.get(name);
                }
            }
        }
        return Err(RuntimeError {
            token: name.clone(),
            message: "Undefined variable: '".to_string() + &name.lexeme + "'.",
        });
    }

    pub fn assign(&mut self, name: &Token, value: &Literal) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());
            return Ok(());
        }
        if let Some(environment) = self.enclosing.as_mut() {
            return environment.assign(name, value);
        }
        return Err(RuntimeError {
            token: name.clone(),
            message: "Undefined variable '".to_string() + &name.lexeme + "'.",
        });
    }
}
