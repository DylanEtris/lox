use crate::{
    error::AstResult,
    token::{Literal, Token},
};

pub trait ExprVisitor<T> {
    fn visit_binary(&mut self, expr: &Expr) -> AstResult<T>;
    fn visit_logical(&mut self, left: &Expr, operator: &Token, right: &Expr) -> AstResult<T>;
    fn visit_grouping(&mut self, expr: &Expr) -> AstResult<T>;
    fn visit_literal(&mut self, expr: &Expr) -> AstResult<T>;
    fn visit_unary(&mut self, expr: &Expr) -> AstResult<T>;
    fn visit_variable(&mut self, expr: &Expr) -> AstResult<T>;
    fn visit_assignment(&mut self, expr: &Expr) -> AstResult<T>;
    fn visit_get_expr(&mut self, object: &Expr, name: &Token) -> AstResult<T>;
    fn visit_this_expr(&mut self, keyword: &Token) -> AstResult<T>;
    fn visit_set_expr(&mut self, object: &Expr, name: &Token, value: &Expr) -> AstResult<T>;
    fn visit_call(
        &mut self,
        callee: &Expr,
        paren: &Token,
        arguments: &Vec<Box<Expr>>,
    ) -> AstResult<T>;
}

#[derive(Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    This {
        keyword: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
    Assignment {
        name: Token,
        value: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Box<Expr>>,
    },
    Get {
        object: Box<Expr>,
        name: Token,
    },
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
    },
}

impl Expr {
    pub fn accept<T: ExprVisitor<U>, U>(&self, visitor: &mut T) -> AstResult<U> {
        match self {
            Self::Binary {
                left: _,
                operator: _,
                right: _,
            } => visitor.visit_binary(self),
            Self::Logical {
                left,
                operator,
                right,
            } => visitor.visit_logical(left, operator, right),
            Self::Grouping { expression: _ } => visitor.visit_grouping(&self),
            Self::Literal { value: _ } => visitor.visit_literal(&self),
            Self::Unary {
                operator: _,
                right: _,
            } => visitor.visit_unary(&self),
            Self::Variable { name: _ } => visitor.visit_variable(&self),
            Expr::Assignment { name: _, value: _ } => visitor.visit_assignment(&self),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => visitor.visit_call(callee, paren, arguments),
            Expr::Get { object, name } => visitor.visit_get_expr(object, name),
            Expr::Set {
                object,
                name,
                value,
            } => visitor.visit_set_expr(object, name, value),
            Expr::This { keyword } => visitor.visit_this_expr(keyword),
        }
    }
}
