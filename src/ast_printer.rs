use crate::expr::{Expr, ExprVisitor};

pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        return expr.accept(self);
    }

    fn parenthesize(&self, name: &str, expressions: &[&Box<Expr>]) -> String {
        let mut result = String::new();
        result.push_str("(");
        result.push_str(&name);
        for expr in expressions {
            result.push_str(" ");
            result.push_str(&expr.accept(self))
        }
        result.push_str(")");
        result
    }
}

impl ExprVisitor<String> for &AstPrinter {
    fn visit_binary(&self, expr: &Expr) -> String {
        let Expr::Binary {
            left,
            operator,
            right,
        } = expr
        else {
            panic!("Expected variant.")
        };
        self.parenthesize(&operator.lexeme, &[left, right])
    }

    fn visit_grouping(&self, expr: &Expr) -> String {
        let Expr::Grouping { expression } = expr else {
            panic!("Expected variant.")
        };
        self.parenthesize("group", &[expression])
    }

    fn visit_literal(&self, expr: &Expr) -> String {
        let Expr::Literal { value } = expr else {
            panic!("Expected variant.")
        };
        value.to_string()
    }

    fn visit_unary(&self, expr: &Expr) -> String {
        let Expr::Unary { operator, right } = expr else {
            panic!("Expected variant.")
        };
        self.parenthesize(&operator.lexeme, &[right])
    }
}
