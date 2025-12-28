use std::fmt::Display;

use crate::token::{Literal, Token};

pub type LoxResult<T> = Result<T, LoxError>;
#[derive(Debug)]
#[must_use = "errors should be handled"]
pub enum RuntimeError {
    Exception(String),
    Return(Literal),
}

impl RuntimeError {
    pub fn from_token(token: Token, message: String) -> RuntimeError {
        RuntimeError::Exception(format!("{}\n[line {}]", message, token.line))
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exception(msg) => write!(f, "{}", msg),
            _ => panic!("Expected variant."),
        }
    }
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
