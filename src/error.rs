use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub struct ErrorLine {
    pub line: usize,
    pub message: String,
}

#[derive(Debug)]
pub enum LoxError {
    ScannerError { errors: Vec<ErrorLine> },
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
