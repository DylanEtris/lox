use crate::{
    Lox,
    error::{LoxError, LoxResult},
    expr::{self, Expr},
    token::{Literal, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<(Token, String)>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Option<Expr>, Vec<(Token, String)>> {
        let result = match self.expression() {
            Ok(expr) => Ok(Some(expr)),
            Err(_) => Ok(None),
        };
        if !self.errors.is_empty() {
            return Err(self.errors.clone());
        }
        result
    }

    fn expression(&mut self) -> Result<Expr, ()> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ()> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Expr, ()> {
        let mut expr = self.term()?;

        while self.match_token(&[
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr, ()> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr, ()> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr, ()> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            return Ok(Expr::Unary {
                operator: operator,
                right: Box::new(self.unary()?),
            });
        } else {
            return self.primary();
        }
    }

    fn primary(&mut self) -> Result<Expr, ()> {
        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::Literal {
                value: Literal::Bool { val: false },
            });
        }
        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::Literal {
                value: Literal::Bool { val: true },
            });
        }
        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expr::Literal {
                value: Literal::Nil,
            });
        }

        if self.match_token(&[TokenType::String, TokenType::Number]) {
            return Ok(Expr::Literal {
                value: self.previous().literal.unwrap(),
            });
        }
        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }
        self.errors
            .push((self.peek(), "Expected expression.".to_string()));
        Err(())
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for type_ in types {
            if self.check(&type_) {
                self.advance();

                return true;
            }
        }
        return false;
    }

    fn consume(&mut self, type_: TokenType, msg: &str) -> Result<(), ()> {
        if self.check(&type_) {
            self.advance();
            return Ok(());
        }
        self.errors.push((self.peek(), msg.to_string()));
        Err(())
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn check(&self, type_: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self.peek().token_type == *type_;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }
            self.advance();
        }
    }
}
