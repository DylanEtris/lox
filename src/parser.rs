use crate::{
    expr::Expr,
    stmt::Stmt,
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<(Token, String)>> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            match self.declaration() {
                Ok(statement) => statements.push(statement),
                Err(_) => {}
            };
        }
        if !self.errors.is_empty() {
            return Err(self.errors.clone());
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, ()> {
        if self.match_token(&[TokenType::Var]) {
            return self.var_declaration();
        }
        if self.match_token(&[TokenType::Fun]) {
            return self.function("function".into());
        }
        return self.statement();
    }

    fn function(&mut self, kind: String) -> Result<Stmt, ()> {
        let name = self.consume(TokenType::Identifier, &format!("Expect {} name.", kind))?;
        self.consume(
            TokenType::LeftParen,
            &format!("Expect '(' after {} name.", &kind),
        )?;
        let mut parameters = Vec::new();
        if !self.check(&TokenType::RightParen) {
            let parameter = self.consume(TokenType::Identifier, "Expect parameter name.")?;
            parameters.push(parameter);
            while self.match_token(&[TokenType::Comma]) {
                let parameter = self.consume(TokenType::Identifier, "Expect parameter name.")?;
                parameters.push(parameter);
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(
            TokenType::LeftBrace,
            &format!("Expect '{{' before {} body.", kind),
        )?;
        let body = self.block()?;
        return Ok(Stmt::Function {
            name: name,
            parameters: parameters,
            body: body,
        });
    }

    fn var_declaration(&mut self) -> Result<Stmt, ()> {
        let name = self.consume(TokenType::Identifier, "Expect an identifier.")?;

        let initializer = if self.match_token(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after a variable declaration.",
        )?;
        Ok(Stmt::Var {
            name: name,
            expression: initializer,
        })
    }

    fn statement(&mut self) -> Result<Stmt, ()> {
        if self.match_token(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_token(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block {
                statements: self.block()?,
            });
        }
        if self.match_token(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.match_token(&[TokenType::While]) {
            return self.while_statement();
        }
        if self.match_token(&[TokenType::For]) {
            return self.for_statement();
        }
        if self.match_token(&[TokenType::Return]) {
            return self.return_statement();
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, ()> {
        let expression = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after a value.")?;
        Ok(Stmt::Print {
            expression: expression,
        })
    }

    fn error(&mut self, at: Token, msg: String) -> Result<(), ()> {
        self.errors.push((at, msg));
        Err(())
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ()> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;

        Ok(statements)
    }

    fn expression_statement(&mut self) -> Result<Stmt, ()> {
        let expression = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after a value.")?;
        Ok(Stmt::Expression {
            expression: expression,
        })
    }

    fn return_statement(&mut self) -> Result<Stmt, ()> {
        let keyword = self.previous();
        let value = if !self.check(&TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expect ';' after a value.")?;
        Ok(Stmt::Return { keyword, value })
    }

    fn if_statement(&mut self) -> Result<Stmt, ()> {
        self.consume(TokenType::LeftParen, "Expect '(' after if.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let if_statement = self.statement()?;
        let else_statement = if self.match_token(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        return Ok(Stmt::If {
            condition,
            if_statement: Box::new(if_statement),
            else_statement: else_statement,
        });
    }

    fn while_statement(&mut self) -> Result<Stmt, ()> {
        self.consume(TokenType::LeftParen, "Expect '(' after while.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let statement = self.statement()?;
        return Ok(Stmt::While {
            condition,
            statement: Box::new(statement),
        });
    }

    fn for_statement(&mut self) -> Result<Stmt, ()> {
        self.consume(TokenType::LeftParen, "Expect '(' after for.")?;
        let initializer = if self.match_token(&[TokenType::Semicolon]) {
            None
        } else if self.match_token(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };
        let condition = if self.match_token(&[TokenType::Semicolon]) {
            Expr::Literal {
                value: Literal::Bool { val: true },
            }
        } else {
            self.expression()?
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if self.check(&TokenType::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Stmt::Block {
                statements: vec![
                    body,
                    Stmt::Expression {
                        expression: increment,
                    },
                ],
            }
        }

        body = Stmt::While {
            condition: condition,
            statement: Box::new(body),
        };

        if let Some(initializer) = initializer {
            body = Stmt::Block {
                statements: vec![initializer, body],
            }
        }
        return Ok(body);
    }

    fn expression(&mut self) -> Result<Expr, ()> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ()> {
        let expr = self.logical_or()?;

        if self.match_token(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.logical_or()?;

            match expr {
                Expr::Variable { name } => {
                    return Ok(Expr::Assignment {
                        name: name,
                        value: Box::new(value),
                    });
                }
                _ => {
                    self.errors
                        .push((equals, "Invalid assignment target.".to_string()));
                }
            }
        }

        Ok(expr)
    }

    fn logical_or(&mut self) -> Result<Expr, ()> {
        let mut expr = self.logical_and()?;
        while self.match_token(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.logical_and()?;

            expr = Expr::Logical {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }
        return Ok(expr);
    }

    fn logical_and(&mut self) -> Result<Expr, ()> {
        let mut expr = self.equality()?;
        while self.match_token(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;

            expr = Expr::Logical {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }
        return Ok(expr);
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
            return self.call();
        }
    }

    fn call(&mut self) -> Result<Expr, ()> {
        let mut expr = self.primary()?;
        loop {
            if self.match_token(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        return Ok(expr);
    }

    fn finish_call(&mut self, callee_: Expr) -> Result<Expr, ()> {
        let mut arguments = Vec::new();
        if !self.check(&TokenType::RightParen) {
            let argument = self.expression()?;
            arguments.push(Box::new(argument));
            while self.match_token(&[TokenType::Comma]) {
                if arguments.len() >= 255 {
                    let _ = self.error(
                        self.peek(),
                        "Can't have more than 255 arguments.".to_string(),
                    );
                }
                let argument = self.expression()?;
                arguments.push(Box::new(argument));
            }
        }
        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;
        return Ok(Expr::Call {
            callee: Box::new(callee_),
            paren: paren,
            arguments: arguments,
        });
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
        if self.match_token(&[TokenType::Identifier]) {
            return Ok(Expr::Variable {
                name: self.previous(),
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

    fn consume(&mut self, type_: TokenType, msg: &str) -> Result<Token, ()> {
        if self.check(&type_) {
            return Ok(self.advance());
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

    //fn synchronize(&mut self) {
    //    self.advance();

    //    while !self.is_at_end() {
    //        if self.previous().token_type == TokenType::Semicolon {
    //            return;
    //        }

    //        match self.peek().token_type {
    //            TokenType::Class
    //            | TokenType::Fun
    //            | TokenType::Var
    //            | TokenType::For
    //            | TokenType::If
    //            | TokenType::While
    //            | TokenType::Print
    //            | TokenType::Return => return,
    //            _ => {}
    //        }
    //        self.advance();
    //    }
    //}
}
