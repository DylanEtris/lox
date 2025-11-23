use crate::expr::{AstResult, Expr, ExprVisitor};

pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        return expr.accept(self).unwrap();
    }

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
    fn visit_binary(&self, expr: &Expr) -> AstResult<String> {
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

    fn visit_grouping(&self, expr: &Expr) -> AstResult<String> {
        let Expr::Grouping { expression } = expr else {
            panic!("Expected variant.")
        };
        Ok(self.parenthesize("group", &[expression]))
    }

    fn visit_literal(&self, expr: &Expr) -> AstResult<String> {
        let Expr::Literal { value } = expr else {
            panic!("Expected variant.")
        };
        Ok(value.to_string())
    }

    fn visit_unary(&self, expr: &Expr) -> AstResult<String> {
        let Expr::Unary { operator, right } = expr else {
            panic!("Expected variant.")
        };
        Ok(self.parenthesize(&operator.lexeme, &[right]))
    }
}
