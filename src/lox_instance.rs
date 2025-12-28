use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::RuntimeError, lox_class::LoxClass, token::Literal};

#[derive(Debug, Clone)]
pub struct LoxInstance {
    klass: LoxClass,
    pub fields: HashMap<String, Literal>,
}

impl LoxInstance {
    pub fn new(klass: LoxClass) -> LoxInstance {
        LoxInstance {
            klass,
            fields: HashMap::new(),
        }
    }

    pub(crate) fn set(
        &mut self,
        name: &crate::token::Token,
        value: Literal,
    ) -> Result<(), RuntimeError> {
        self.fields.insert(name.lexeme.clone(), value);
        Ok(())
    }
}

pub(crate) fn instance_get(
    instance: &Rc<RefCell<LoxInstance>>,
    name: &crate::token::Token,
) -> Result<crate::token::Literal, RuntimeError> {
    if let Some(val) = instance.borrow().fields.get(&name.lexeme) {
        return Ok(val.clone());
    }
    let method = instance.borrow().klass.find_method(&name.lexeme);
    if let Some(method) = method {
        let method = method.bind(instance.clone());
        let method = Rc::new(method);
        return Ok(Literal::Callable(method));
    }
    return Err(RuntimeError::from_token(
        name.clone(),
        format!("Undefined property {}.", name.lexeme),
    ));
}

impl ToString for LoxInstance {
    fn to_string(&self) -> String {
        format!("{} instance", self.klass.name)
    }
}
