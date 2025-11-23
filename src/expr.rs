use crate::token::{Literal, Token};

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

pub type AstResult<T> = Result<T, RuntimeError>;

pub trait ExprVisitor<T> {
    fn visit_binary(&self, expr: &Expr) -> AstResult<T>;
    fn visit_grouping(&self, expr: &Expr) -> AstResult<T>;
    fn visit_literal(&self, expr: &Expr) -> AstResult<T>;
    fn visit_unary(&self, expr: &Expr) -> AstResult<T>;
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
}

impl Expr {
    pub fn accept<T: ExprVisitor<U>, U>(&self, visitor: T) -> AstResult<U> {
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
        }
    }
}
