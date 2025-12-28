use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{
    environment::Environment,
    error::{AstResult, RuntimeError},
    interpreter::Interpreter,
    lox_callable::LoxCallable,
    lox_instance::LoxInstance,
    stmt::Stmt,
    token::{Literal, Token},
};

#[derive(Clone)]
pub struct LoxFunction {
    pub name: Token,
    pub parameters: Vec<Token>,
    pub body: Vec<Stmt>,
    pub closure: Rc<RefCell<Environment>>,
    pub is_initializer: bool,
}

impl LoxFunction {
    pub fn bind(&self, instance: Rc<RefCell<LoxInstance>>) -> LoxFunction {
        let mut environment = Environment::new(Rc::clone(&self.closure));
        environment.define("this".into(), Literal::Instance(instance));
        LoxFunction {
            name: self.name.clone(),
            parameters: self.parameters.to_vec(),
            body: self.body.to_vec(),
            closure: Rc::new(RefCell::new(environment)),
            is_initializer: self.is_initializer,
        }
    }
}

impl Debug for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoxFunction")
            .field("name", &self.name)
            .field("parameters", &self.parameters)
            .finish()
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Literal>) -> AstResult<Literal> {
        let mut environment = Environment::new(Rc::clone(&self.closure));
        for (i, param) in self.parameters.iter().enumerate() {
            environment.define(param.lexeme.clone(), arguments[i].clone())
        }
        match interpreter.execute_block(&self.body, environment) {
            Ok(()) => {
                if self.is_initializer {
                    return Ok(self.closure.borrow().get_at(0, "this")?);
                }
                Ok(Literal::Nil)
            }
            Err(RuntimeError::Return(value)) => {
                if self.is_initializer {
                    return Ok(self.closure.borrow().get_at(0, "this")?);
                }
                Ok(value)
            }
            Err(e) => Err(e),
        }
    }

    fn arity(&self) -> usize {
        self.parameters.len()
    }
}

impl ToString for LoxFunction {
    fn to_string(&self) -> String {
        return format!("<fn {}>", self.name.lexeme);
    }
}
