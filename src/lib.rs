//! blah
#![deny(clippy::pedantic)]
//#![deny(missing_docs)]

pub mod error;
pub mod lox;
pub mod scanner;
pub mod token;

pub use crate::lox::Lox;
