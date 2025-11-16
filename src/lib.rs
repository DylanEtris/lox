//! blah
#![deny(clippy::pedantic)]
//#![deny(missing_docs)]

pub mod ast_printer;
pub mod error;
pub mod expr;
pub mod lox;
pub mod scanner;
pub mod token;

pub use crate::lox::Lox;
