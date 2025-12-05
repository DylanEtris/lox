use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::Environment,
    error::{AstResult, RuntimeError},
    interpreter::Interpreter,
    lox_callable::LoxCallable,
    stmt::Stmt,
    token::{Literal, Token},
};

pub struct LoxFunction {
    pub name: Token,
    pub parameters: Vec<Token>,
    pub body: Vec<Stmt>,
    pub closure: Rc<RefCell<Environment>>,
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Literal>) -> AstResult<Literal> {
        let mut environment = Environment::new(Rc::clone(&self.closure));
        for (i, param) in self.parameters.iter().enumerate() {
            environment.define(param.lexeme.clone(), arguments[i].clone())
        }
        match interpreter.execute_block(&self.body, environment) {
            Ok(()) => Ok(Literal::Nil),
            Err(RuntimeError::Return(value)) => Ok(value),
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
