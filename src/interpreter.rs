use crate::{
    environment::{Environment},
    error::{AstResult, RuntimeError},
    expr::{Expr, ExprVisitor},
    stmt::{Stmt, StmtVisitor},
    token::{Literal, Token, TokenType},
};

#[derive(Default)]
pub struct Interpreter {
    environment: Environment,
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

    fn is_truthy(&self, value: &Literal) -> Literal {
        match value {
            Literal::Nil => Literal::Bool { val: false },
            Literal::Bool { val } => Literal::Bool { val: *val },
            _ => Literal::Bool { val: true },
        }
    }

    fn check_number_operand(&self, operator: Token, value: &Literal) -> Result<(), RuntimeError> {
        match value {
            Literal::Number { val: _ } => Ok(()),
            _ => Err(RuntimeError {
                token: operator,
                message: "Operand must be a number.".to_string(),
            }),
        }
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
                token: operator,
                message: "Operand must be a number.".to_string(),
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
                token: operator,
                message: "Operand must be a number.".to_string(),
            }),
        }
    }

    fn execute_block(
        &mut self,
        statements: &Vec<Box<Stmt>>,
        environment: Environment,
    ) -> AstResult<()> {
        let previous = self.environment.clone();
        self.environment = environment;

        let mut result = Ok(());
        for statement in statements {
            result = self.execute(statement);
        }

        self.environment = previous;
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
                return Ok(self.is_truthy(&value));
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
        self.environment.get(name)
    }

    fn visit_assignment(&mut self, expr: &Expr) -> AstResult<Literal> {
        let Expr::Assignment { name, value } = expr else {
            panic!("Expected variant.")
        };
        let value = self.evaluate(value)?;
        self.environment.assign(&name, &value)?;
        Ok(value)
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
        self.environment.define(name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &Stmt) -> AstResult<()> {
        let Stmt::Block { statements } = stmt else {
            panic!("Expected variant.")
        };
        self.execute_block(
            statements,
            Environment::new(Box::new(self.environment.clone())),
        )
    }
}
