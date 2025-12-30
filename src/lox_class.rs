use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{lox_callable::LoxCallable, lox_function::LoxFunction, lox_instance::LoxInstance};

#[derive(Clone, Debug)]
pub struct LoxClass {
    pub name: String,
    methods: HashMap<String, Rc<LoxFunction>>,
    superclass: Option<Rc<LoxClass>>,
}

impl LoxClass {
    pub fn new(name: String, methods: HashMap<String, Rc<LoxFunction>>) -> LoxClass {
        LoxClass {
            name,
            methods,
            superclass: None,
        }
    }

    pub fn superclass(mut self, superclass: Option<Rc<LoxClass>>) -> LoxClass {
        self.superclass = superclass;
        self
    }

    pub(crate) fn find_method(&self, lexeme: &str) -> Option<Rc<LoxFunction>> {
        if self.methods.contains_key(lexeme) {
            let function = Rc::clone(self.methods.get(lexeme)?);
            return Some(function);
        }
        if let Some(superclass) = &self.superclass {
            return superclass.find_method(lexeme);
        }
        None
    }
}

impl ToString for LoxClass {
    fn to_string(&self) -> String {
        format!("{}", self.name)
    }
}

impl LoxCallable for LoxClass {
    fn call(
        &self,
        interpreter: &mut crate::interpreter::Interpreter,
        arguments: Vec<crate::token::Literal>,
    ) -> crate::error::AstResult<crate::token::Literal> {
        let instance = Rc::new(RefCell::new(LoxInstance::new(self.clone())));
        if let Some(initializer) = self.find_method("init") {
            initializer
                .bind(Rc::clone(&instance))
                .call(interpreter, arguments)?;
        }

        Ok(crate::token::Literal::Instance(instance))
    }

    fn arity(&self) -> usize {
        match self.find_method("init") {
            Some(initializer) => initializer.arity(),
            None => 0,
        }
    }
}
