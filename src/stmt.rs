use crate::{error::AstResult, expr::Expr, token::Token};

pub trait StmtVisitor {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> AstResult<()>;
    fn visit_print_stmt(&mut self, stmt: &Stmt) -> AstResult<()>;
    fn visit_var_stmt(&mut self, stmt: &Stmt) -> AstResult<()>;
    fn visit_block_stmt(&mut self, stmt: &Stmt) -> AstResult<()>;
    fn visit_if_stmt(
        &mut self,
        expression: &Expr,
        if_stmt: &Stmt,
        else_stmt: Option<&Stmt>,
    ) -> AstResult<()>;
    fn visit_while(&mut self, condition: &Expr, statement: &Stmt) -> AstResult<()>;
    fn visit_function(
        &mut self,
        name: &Token,
        params: &Vec<Token>,
        body: &Vec<Stmt>,
    ) -> AstResult<()>;
    fn visit_return(&mut self, keyword: &Token, value: &Option<Expr>) -> AstResult<()>;
    fn visit_class(
        &mut self,
        name: &Token,
        methods: &Vec<Stmt>,
        superclass: &Option<Token>,
    ) -> AstResult<()>;
}

#[derive(Clone)]
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
    Function {
        name: Token,
        parameters: Vec<Token>,
        body: Vec<Stmt>,
    },
    Block {
        statements: Vec<Stmt>,
    },
    Class {
        name: Token,
        methods: Vec<Stmt>,
        superclass: Option<Token>,
    },
    If {
        condition: Expr,
        if_statement: Box<Stmt>,
        else_statement: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        statement: Box<Stmt>,
    },
    Return {
        keyword: Token,
        value: Option<Expr>,
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
            Self::If {
                condition: expression,
                if_statement,
                else_statement,
            } => visitor.visit_if_stmt(&expression, if_statement, else_statement.as_deref()),
            Self::While {
                condition,
                statement,
            } => visitor.visit_while(condition, statement),
            Self::Function {
                name,
                parameters,
                body,
            } => visitor.visit_function(name, parameters, body),
            Self::Return { keyword, value } => visitor.visit_return(keyword, value),
            Self::Class {
                name,
                methods,
                superclass,
            } => visitor.visit_class(name, methods, superclass),
        }
    }
}
