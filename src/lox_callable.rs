use crate::{error::AstResult, interpreter::Interpreter, token::Literal};

pub trait LoxCallable {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Literal>) -> AstResult<Literal>;
    fn arity(&self) -> usize;
}
