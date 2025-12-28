use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    error::RuntimeError,
    token::{Literal, Token},
};

#[derive(Default, Clone)]
pub struct Environment {
    values: HashMap<String, Literal>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            values: HashMap::default(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<Literal, RuntimeError> {
        match self.values.get(name) {
            Some(literal) => {
                return Ok(literal.clone());
            }
            None => {
                if let Some(environment) = &self.enclosing {
                    return environment.borrow().get(name);
                }
            }
        }
        return Err(RuntimeError::Exception(format!(
            "Undefined variable: '{}'.",
            name
        )));
    }

    pub fn get_at(&self, distance: usize, name: &str) -> Result<Literal, RuntimeError> {
        if distance == 0 {
            self.get(name)
        } else {
            let Some(enclosing) = self.enclosing.as_ref() else {
                return Err(RuntimeError::Exception("Variable does not exist.".into()));
            };
            return enclosing.borrow().get_at(distance - 1, name);
        }
    }

    pub fn assign(&mut self, name: &Token, value: &Literal) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());
            return Ok(());
        }
        if let Some(environment) = self.enclosing.as_mut() {
            return environment.borrow_mut().assign(name, value);
        }
        return Err(RuntimeError::from_token(
            name.clone(),
            format!("Undefined variable '{}'.", &name.lexeme),
        ));
    }

    pub fn assign_at(
        &mut self,
        distance: usize,
        name: &Token,
        value: &Literal,
    ) -> Result<(), RuntimeError> {
        if distance == 0 {
            self.assign(name, value)
        } else {
            let Some(enclosing) = self.enclosing.as_ref() else {
                return Err(RuntimeError::from_token(
                    name.clone(),
                    "Variable does not exist.".into(),
                ));
            };
            return enclosing.borrow_mut().assign_at(distance - 1, name, value);
        }
    }
}
