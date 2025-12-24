use crate::error::AstResult;
use crate::expr::{Expr, ExprVisitor};
use crate::stmt::Stmt;

#[derive(Default)]
pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(&mut self, stmt: &Vec<Stmt>) -> String {}

    fn parenthesize(&self, name: &str, expressions: &[&Box<Expr>]) -> String {
        let mut result = String::new();
        result.push_str("(");
        result.push_str(&name);
        for expr in expressions {
            result.push_str(" ");
            result.push_str(&expr.accept(self).unwrap())
        }
        result.push_str(")");
        result
    }
}

impl ExprVisitor<String> for &AstPrinter {
    fn visit_binary(&mut self, expr: &Expr) -> AstResult<String> {
        let Expr::Binary {
            left,
            operator,
            right,
        } = expr
        else {
            panic!("Expected variant.")
        };
        Ok(self.parenthesize(&operator.lexeme, &[left, right]))
    }

    fn visit_grouping(&mut self, expr: &Expr) -> AstResult<String> {
        let Expr::Grouping { expression } = expr else {
            panic!("Expected variant.")
        };
        Ok(self.parenthesize("group", &[expression]))
    }

    fn visit_literal(&mut self, expr: &Expr) -> AstResult<String> {
        let Expr::Literal { value } = expr else {
            panic!("Expected variant.")
        };
        Ok(value.to_string())
    }

    fn visit_unary(&mut self, expr: &Expr) -> AstResult<String> {
        let Expr::Unary { operator, right } = expr else {
            panic!("Expected variant.")
        };
        Ok(self.parenthesize(&operator.lexeme, &[right]))
    }

    fn visit_logical(
        &mut self,
        left: &Expr,
        operator: &crate::token::Token,
        right: &Expr,
    ) -> crate::error::AstResult<String> {
        todo!()
    }

    fn visit_variable(&mut self, expr: &Expr) -> crate::error::AstResult<String> {
        todo!()
    }

    fn visit_assignment(&mut self, expr: &Expr) -> crate::error::AstResult<String> {
        todo!()
    }

    fn visit_call(
        &mut self,
        callee: &Expr,
        paren: &crate::token::Token,
        arguments: &Vec<Box<Expr>>,
    ) -> crate::error::AstResult<String> {
        todo!()
    }
}
