use crate::{
    environment::Environment,
    error::{AstResult, RuntimeError},
    expr::{Expr, ExprVisitor},
    stmt::{Stmt, StmtVisitor},
    token::{Literal, Token, TokenType},
};

pub struct Interpreter {
    environment: Option<Environment>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            environment: Some(Environment::default()),
        }
    }
}

impl Interpreter {
    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> AstResult<()> {
        for stmt in stmts {
            self.execute(&stmt)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> AstResult<()> {
        stmt.accept(self)
    }

    fn evaluate(&mut self, expr: &Expr) -> AstResult<Literal> {
        expr.accept(self)
    }

    fn is_truthy(&self, value: &Literal) -> bool {
        match value {
            Literal::Nil => false,
            Literal::Bool { val } => *val,
            _ => true,
        }
    }

    fn check_number_operand(&self, operator: Token, value: &Literal) -> Result<(), RuntimeError> {
        let val = match value {
            Literal::Number { val: _ } => Ok(()),
            _ => Err(RuntimeError {
                token: operator,
                message: "Operand must be a number.".to_string(),
            }),
        };
        return val;
    }

    fn check_number_operands(
        &self,
        operator: Token,
        left: &Literal,
        right: &Literal,
    ) -> Result<(), RuntimeError> {
        match (left, right) {
            (Literal::Number { val: _ }, Literal::Number { val: _ }) => Ok(()),
            _ => Err(RuntimeError {
                token: operator.clone(),
                message: "Operand for '".to_string() + &operator.lexeme + "' must be a number.",
            }),
        }
    }

    fn check_number_or_string_operands(
        &self,
        operator: Token,
        left: &Literal,
        right: &Literal,
    ) -> Result<(), RuntimeError> {
        match (left, right) {
            (Literal::Number { val: _ }, Literal::Number { val: _ }) => Ok(()),
            (Literal::String { val: _ }, Literal::String { val: _ }) => Ok(()),
            _ => Err(RuntimeError {
                token: operator.clone(),
                message: format!(
                    "Operand for '{}' must be a number or a string.\nGot {:?} and {:?}.",
                    operator.lexeme, left, right
                ),
            }),
        }
    }

    fn execute_block(&mut self, statements: &Vec<Box<Stmt>>) -> AstResult<()> {
        let previous = self.environment.take().unwrap();
        self.environment = Some(Environment::new(Box::new(previous)));

        let mut result = Ok(());
        for statement in statements {
            result = self.execute(statement);
        }

        let current = self.environment.take().unwrap();
        self.environment = Some(*current.enclosing.unwrap());
        result
    }
}

impl ExprVisitor<Literal> for Interpreter {
    fn visit_binary(&mut self, expr: &Expr) -> AstResult<Literal> {
        let Expr::Binary {
            left,
            operator,
            right,
        } = expr
        else {
            panic!("Expected binary.")
        };
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match operator.token_type {
            TokenType::Minus => {
                self.check_number_operands(operator.clone(), &left, &right)?;
                Ok(left - right)
            }
            TokenType::Slash => {
                self.check_number_operands(operator.clone(), &left, &right)?;
                Ok(left / right)
            }
            TokenType::Star => {
                self.check_number_operands(operator.clone(), &left, &right)?;
                Ok(left * right)
            }
            TokenType::Plus => {
                self.check_number_or_string_operands(operator.clone(), &left, &right)?;
                Ok(left + right)
            }
            TokenType::Less => {
                self.check_number_operands(operator.clone(), &left, &right)?;
                Ok(Literal::Bool { val: left < right })
            }
            TokenType::LessEqual => {
                self.check_number_operands(operator.clone(), &left, &right)?;
                Ok(Literal::Bool { val: left <= right })
            }
            TokenType::Greater => {
                self.check_number_operands(operator.clone(), &left, &right)?;
                Ok(Literal::Bool { val: left > right })
            }
            TokenType::GreaterEqual => {
                self.check_number_operands(operator.clone(), &left, &right)?;
                Ok(Literal::Bool { val: left >= right })
            }
            TokenType::EqualEqual => {
                self.check_number_operands(operator.clone(), &left, &right)?;
                Ok(Literal::Bool { val: left == right })
            }
            TokenType::BangEqual => {
                self.check_number_operands(operator.clone(), &left, &right)?;
                Ok(Literal::Bool { val: left != right })
            }
            _ => Ok(Literal::Nil),
        }
    }

    fn visit_grouping(&mut self, expr: &Expr) -> AstResult<Literal> {
        let Expr::Grouping { expression } = expr else {
            panic!("Expected variant.")
        };
        self.evaluate(expression)
    }

    fn visit_literal(&mut self, expr: &Expr) -> AstResult<Literal> {
        let Expr::Literal { value } = expr else {
            panic!("Expected variant.")
        };
        Ok(value.clone())
    }

    fn visit_unary(&mut self, expr: &Expr) -> AstResult<Literal> {
        let Expr::Unary { operator, right } = expr else {
            panic!("Expected variant.")
        };
        let value = self.evaluate(right)?;
        match operator.token_type {
            TokenType::Bang => {
                return Ok(Literal::Bool {
                    val: self.is_truthy(&value),
                });
            }
            TokenType::Minus => {
                self.check_number_operand(operator.clone(), &value)?;
                return Ok(-value);
            }
            _ => {
                return Ok(Literal::Nil);
            }
        }
    }

    fn visit_variable(&mut self, expr: &Expr) -> AstResult<Literal> {
        let Expr::Variable { name } = expr else {
            panic!("Expected variant.")
        };
        self.environment.as_mut().unwrap().get(name)
    }

    fn visit_assignment(&mut self, expr: &Expr) -> AstResult<Literal> {
        let Expr::Assignment { name, value } = expr else {
            panic!("Expected variant.")
        };
        let value = self.evaluate(value)?;
        self.environment.as_mut().unwrap().assign(&name, &value)?;
        Ok(value)
    }

    fn visit_logical(&mut self, left: &Expr, operator: &Token, right: &Expr) -> AstResult<Literal> {
        match operator.token_type {
            TokenType::And => {
                let left = self.evaluate(left)?;
                if !self.is_truthy(&left) {
                    return Ok(left);
                }
            }
            TokenType::Or => {
                let left = self.evaluate(left)?;
                if self.is_truthy(&left) {
                    return Ok(left);
                }
            }
            _ => panic!("Expected an 'and' or an 'or'"),
        }
        self.evaluate(&right)
    }

    fn visit_call(
        &mut self,
        name: &Token,
        paren: &Token,
        arguments: &Vec<Box<Expr>>,
    ) -> AstResult<Literal> {
        todo!()
    }
}

impl StmtVisitor for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> AstResult<()> {
        let Stmt::Expression { expression } = stmt else {
            panic!("Expected variant.")
        };
        self.evaluate(expression)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) -> AstResult<()> {
        let Stmt::Print { expression } = stmt else {
            panic!("Expected variant.")
        };
        let value = self.evaluate(expression)?;
        println!("{}", value.to_string());
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) -> AstResult<()> {
        let Stmt::Var { name, expression } = stmt else {
            panic!("Expected variant.")
        };
        let value = match expression {
            Some(expr) => self.evaluate(expr)?,
            None => Literal::Nil,
        };
        self.environment
            .as_mut()
            .unwrap()
            .define(name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &Stmt) -> AstResult<()> {
        let Stmt::Block { statements } = stmt else {
            panic!("Expected variant.")
        };
        self.execute_block(statements)
    }

    fn visit_if_stmt(
        &mut self,
        expression: &Expr,
        if_stmt: &Stmt,
        else_stmt: Option<&Stmt>,
    ) -> AstResult<()> {
        let condition = self.evaluate(expression)?;
        if self.is_truthy(&condition) {
            self.execute(if_stmt)?;
        } else if let Some(else_stmt) = else_stmt {
            self.execute(else_stmt)?;
        }
        Ok(())
    }

    fn visit_while(&mut self, condition: &Expr, statement: &Stmt) -> AstResult<()> {
        let mut guard = self.evaluate(condition)?;
        while self.is_truthy(&guard) {
            self.execute(statement)?;
            guard = self.evaluate(condition)?;
        }
        Ok(())
    }
}
