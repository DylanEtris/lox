//! blah
use crate::{error::LoxError, scanner::Scanner, token::Token};
use std::{
    collections::HashMap,
    fs::File,
    io::{Error, Read, Write, stderr, stdin, stdout},
    process::exit,
};

/// Main struct for the Lox compiler
#[derive(Default)]
pub struct Lox {
    had_error: bool,
}

impl Lox {
    /// Run the Lox interpreter
    pub fn main(&mut self, args: Vec<String>) -> Result<(), Error> {
        if args.len() > 1 {
            println!("Usage: jlox [script]");
            exit(64);
        } else if args.len() == 1 {
            self.run_file(args[0].clone())?;
        } else {
            self.run_prompt();
        }
        Ok(())
    }

    /// Consume the given file's contents and run it
    pub fn run_file(&mut self, file: String) -> Result<(), Error> {
        let mut file = File::open(file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        self.run(contents);
        if self.had_error {
            exit(65);
        }
        Ok(())
    }

    /// Run an interactive prompt for line-by-line input
    pub fn run_prompt(&mut self) {
        let stdin = stdin();
        let mut input = String::new();
        loop {
            print!("> ");
            let _ = stdout().flush();
            match stdin.read_line(&mut input) {
                Ok(_) => self.run(input.clone()),
                Err(_) => break,
            };
            self.had_error = false;
            input.clear();
        }
    }

    /// Run a string of lox
    pub fn run(&mut self, source: String) {
        let scanner = Scanner::new(source);
        match scanner.scan_tokens() {
            Ok(tokens) => {
                for token in tokens {
                    println!("{}", token);
                }
            }
            Err(e) => self.error(e).unwrap(),
        };
    }

    fn error(&mut self, lox_error: LoxError) -> Result<(), Error> {
        match lox_error {
            LoxError::ScannerError { errors } => {
                for error in errors {
                    self.report(error.line, String::new(), error.message)?
                }
            }
        }
        Ok(())
    }

    fn report(&mut self, line: usize, where_at: String, message: String) -> Result<(), Error> {
        stderr().write_all(format!("[line {}] Error{}: {}", line, where_at, message).as_bytes())?;
        self.had_error = true;
        Ok(())
    }
}
