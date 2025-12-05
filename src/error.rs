use crate::token::{Literal, Token};

pub type LoxResult<T> = Result<T, LoxError>;
#[derive(Debug)]
pub enum RuntimeError {
    Exception { token: Token, message: String },
    Return(Literal),
}

pub type AstResult<T> = Result<T, RuntimeError>;

/// Error line class
#[derive(Debug, Clone)]
pub struct ErrorLine {
    /// Line number
    pub line: usize,

    /// Error message
    pub message: String,
}

/// Lox error enum
#[derive(Debug)]
pub enum LoxError {
    /// Errors for the Scanner class
    ScannerError {
        /// All errors the scanner encountered
        errors: Vec<ErrorLine>,
    },
    ParseError {
        /// All errors the Parser encountered
        error: (Token, String),
    },
}
