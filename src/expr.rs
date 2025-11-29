use crate::{
    error::AstResult,
    token::{Literal, Token},
};

pub trait ExprVisitor<T> {
    fn visit_binary(&mut self, expr: &Expr) -> AstResult<T>;
    fn visit_grouping(&mut self, expr: &Expr) -> AstResult<T>;
    fn visit_literal(&mut self, expr: &Expr) -> AstResult<T>;
    fn visit_unary(&mut self, expr: &Expr) -> AstResult<T>;
    fn visit_variable(&mut self, expr: &Expr) -> AstResult<T>;
    fn visit_assignment(&mut self, expr: &Expr) -> AstResult<T>;
}

pub enum Expr {
    Binary {
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
}

impl Expr {
    pub fn accept<T: ExprVisitor<U>, U>(&self, visitor: &mut T) -> AstResult<U> {
        match self {
            Self::Binary {
                left: _,
                operator: _,
                right: _,
            } => visitor.visit_binary(self),
            Self::Grouping { expression: _ } => visitor.visit_grouping(&self),
            Self::Literal { value: _ } => visitor.visit_literal(&self),
            Self::Unary {
                operator: _,
                right: _,
            } => visitor.visit_unary(&self),
            Self::Variable { name: _ } => visitor.visit_variable(&self),
            Expr::Assignment { name: _, value: _ } => visitor.visit_assignment(&self),
        }
    }
}
