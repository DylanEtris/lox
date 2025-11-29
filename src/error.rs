use crate::token::Token;

pub type LoxResult<T> = Result<T, LoxError>;
#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
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
