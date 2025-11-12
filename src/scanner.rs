//! The definition for the Lox scanner
use std::collections::HashMap;

use crate::{
    error::{ErrorLine, LoxError},
    token::{Literal, Token, TokenType},
};

/// Blah
pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
    errors: Vec<ErrorLine>,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    /// Create a new Scanner
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
            errors: Vec::new(),
            keywords: HashMap::from([
                (String::from("and"), TokenType::And),
                (String::from("class"), TokenType::Class),
                (String::from("else"), TokenType::Else),
                (String::from("false"), TokenType::False),
                (String::from("for"), TokenType::For),
                (String::from("fun"), TokenType::Fun),
                (String::from("if"), TokenType::If),
                (String::from("nil"), TokenType::Nil),
                (String::from("or"), TokenType::Or),
                (String::from("print"), TokenType::Print),
                (String::from("return"), TokenType::Return),
                (String::from("super"), TokenType::Super),
                (String::from("this"), TokenType::This),
                (String::from("true"), TokenType::True),
                (String::from("var"), TokenType::Var),
                (String::from("while"), TokenType::While),
            ]),
        }
    }

    /// Scan the tokens stored in this object
    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LoxError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }
        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        if self.errors.is_empty() {
            Ok(self.tokens.clone())
        } else {
            Err(LoxError::ScannerError {
                errors: self.errors.clone(),
            })
        }
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let matches = self.match_next('=');
                self.add_token(if matches {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                });
            }
            '=' => {
                let matches = self.match_next('=');
                self.add_token(if matches {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                });
            }
            '<' => {
                let matches = self.match_next('=');
                self.add_token(if matches {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                });
            }
            '>' => {
                let matches = self.match_next('=');
                self.add_token(if matches {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                });
            }
            '/' => {
                let matches = self.match_next('/');
                if matches {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' => {
                // Do nothing
            }
            '\r' => {
                // Do nothing
            }
            '\t' => {
                // Do nothing
            }
            '\n' => self.line += 1,
            '"' => self.string(),
            c => {
                if self.is_digit(c) {
                    self.digit();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    self.errors.push(ErrorLine {
                        line: self.line,
                        message: "unexpected character".to_string(),
                    });
                }
            }
        }
    }

    fn match_next(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.as_bytes()[self.current] as char != c {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn is_alpha(&self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_');
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        return self.is_alpha(c) || self.is_digit(c);
    }

    fn digit(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let val = self.source[self.start..self.current]
            .to_string()
            .parse()
            .unwrap();
        self.add_token_literal(TokenType::Number, Literal::Number { val });
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let token_type = match self
            .keywords
            .get(&self.source[self.start..self.current].to_string())
        {
            Some(t) => t.clone(),
            None => TokenType::Identifier,
        };

        self.add_token(token_type);
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors.push(ErrorLine {
                line: self.line,
                message: "Unterminated string.".to_string(),
            })
        }

        self.advance();

        let val: String = self.source[(self.start + 1)..(self.current - 1)].to_string();
        self.add_token_literal(TokenType::String, Literal::String { val });
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.as_bytes()[self.current + 1] as char;
    }

    fn advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current];
        self.current += 1;
        return c as char;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.as_bytes()[self.current] as char;
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(
            token_type,
            self.source[self.start..self.current].to_string(),
            None,
            self.line,
        ))
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Literal) {
        self.tokens.push(Token::new(
            token_type,
            self.source[self.start..self.current].to_string(),
            Some(literal),
            self.line,
        ))
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
