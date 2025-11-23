use crate::{
    expr::{AstResult, Expr, ExprVisitor, RuntimeError},
    token::{Literal, Token, TokenType},
};

#[derive(Default)]
pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(&self, expr: Expr) -> AstResult<()> {
        let value = self.evaluate(&expr)?;
        println!("{}", value.to_string());
        Ok(())
    }

    fn evaluate(&self, expr: &Expr) -> AstResult<Literal> {
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
}

impl ExprVisitor<Literal> for &Interpreter {
    fn visit_binary(&self, expr: &Expr) -> AstResult<Literal> {
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

    fn visit_grouping(&self, expr: &Expr) -> AstResult<Literal> {
        let Expr::Grouping { expression } = expr else {
            panic!("Expected variant.")
        };
        self.evaluate(expression)
    }

    fn visit_literal(&self, expr: &Expr) -> AstResult<Literal> {
        let Expr::Literal { value } = expr else {
            panic!("Expected variant.")
        };
        Ok(value.clone())
    }

    fn visit_unary(&self, expr: &Expr) -> AstResult<Literal> {
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
}
