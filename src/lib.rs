//! blah
#![deny(clippy::pedantic)]
//#![deny(missing_docs)]

//pub mod ast_printer;
pub mod environment;
pub mod error;
pub mod expr;
pub mod interpreter;
pub mod lox;
pub mod lox_callable;
pub mod lox_function;
pub mod parser;
pub mod resolver;
pub mod scanner;
pub mod stmt;
pub mod token;

pub use crate::lox::Lox;
