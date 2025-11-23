//! blah
use crate::{
    error::LoxError,
    expr::RuntimeError,
    interpreter::{Interpreter},
    parser::Parser,
    scanner::Scanner,
    token::{Token, TokenType},
};
use std::{
    fs::File,
    io::{Error, Read, Write, stderr, stdin, stdout},
    process::exit,
};

/// Main struct for the Lox compiler
#[derive(Default)]
pub struct Lox {
    had_error: bool,
    had_runtime_error: bool,
    interpreter: Interpreter,
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
        if self.had_runtime_error {
            exit(70);
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
        let mut scanner = Scanner::new(source);
        let tokens = match scanner.scan_tokens() {
            Ok(tokens) => tokens,
            Err(e) => {
                self.error(e).unwrap();
                return;
            }
        };
        let mut parser = Parser::new(tokens);
        let expression = match parser.parse() {
            Ok(expr) => expr.unwrap(),
            Err(e) => {
                for error in e {
                    self.report_parse_error(error.0, error.1);
                }
                return;
            }
        };
        let _ = match self.interpreter.interpret(expression) {
            Ok(()) => (),
            Err(e) => self.runtime_error(e),
        };
    }

    fn runtime_error(&mut self, error: RuntimeError) {
        println!("{}\n[line {}]", error.message, error.token.line);
        self.had_runtime_error = true;
    }

    fn error(&mut self, lox_error: LoxError) -> Result<(), Error> {
        match lox_error {
            LoxError::ScannerError { errors } => {
                for error in errors {
                    self.report(error.line, String::new(), error.message)?
                }
            }
            LoxError::ParseError { error } => self.report_parse_error(error.0, error.1),
        }
        Ok(())
    }

    fn report(&mut self, line: usize, where_at: String, message: String) -> Result<(), Error> {
        stderr()
            .write_all(format!("[line {}] Error{}: {}\n", line, where_at, message).as_bytes())?;
        self.had_error = true;
        Ok(())
    }

    fn report_parse_error(&mut self, token: Token, message: String) {
        if token.token_type == TokenType::Eof {
            self.report(token.line, " at end".to_string(), message)
                .unwrap();
        } else {
            self.report(
                token.line,
                " at '".to_string() + &token.lexeme + "'",
                message,
            )
            .unwrap();
        }
    }
}
