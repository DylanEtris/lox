use crate::{error::AstResult, expr::Expr, token::Token};

pub trait StmtVisitor {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> AstResult<()>;
    fn visit_print_stmt(&mut self, stmt: &Stmt) -> AstResult<()>;
    fn visit_var_stmt(&mut self, stmt: &Stmt) -> AstResult<()>;
    fn visit_block_stmt(&mut self, stmt: &Stmt) -> AstResult<()>;
}

pub enum Stmt {
    Expression {
        expression: Expr,
    },
    Print {
        expression: Expr,
    },
    Var {
        name: Token,
        expression: Option<Expr>,
    },
    Block {
        statements: Vec<Box<Stmt>>,
    },
}

impl Stmt {
    pub fn accept<T: StmtVisitor>(&self, visitor: &mut T) -> AstResult<()> {
        match self {
            Self::Print { expression: _ } => visitor.visit_print_stmt(self),
            Self::Expression { expression: _ } => visitor.visit_expression_stmt(&self),
            Self::Var {
                name: _,
                expression: _,
            } => visitor.visit_var_stmt(&self),
            Self::Block { statements: _ } => visitor.visit_block_stmt(&self),
        }
    }
}
