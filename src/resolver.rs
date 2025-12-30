use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    error::{AstResult, RuntimeError},
    expr::{Expr, ExprVisitor},
    interpreter::Interpreter,
    stmt::{Stmt, StmtVisitor},
    token::Token,
};

#[derive(Clone)]
enum FunctionType {
    None,
    Function,
    Initializer,
    Method,
}

#[derive(Clone)]
enum ClassType {
    None,
    Class,
    Subclass,
}

pub struct Resolver {
    interpreter: Rc<RefCell<Interpreter>>,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    enclosing_class: ClassType,
}

impl Resolver {
    pub fn new(interpreter: Rc<RefCell<Interpreter>>) -> Resolver {
        Resolver {
            interpreter,
            scopes: Vec::default(),
            current_function: FunctionType::None,
            enclosing_class: ClassType::None,
        }
    }

    pub fn resolve(&mut self, statements: &Vec<Stmt>) -> AstResult<()> {
        for statement in statements {
            self.resolve_stmt(statement)?
        }
        Ok(())
    }

    fn resolve_stmt(&mut self, statement: &Stmt) -> AstResult<()> {
        statement.accept(self)
    }

    fn resolve_expr(&mut self, expr: &Expr) -> AstResult<()> {
        expr.accept(self)
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) -> AstResult<()> {
        if self.scopes.is_empty() {
            return Ok(());
        }
        if self.scopes.last_mut().unwrap().contains_key(&name.lexeme) {
            return Err(RuntimeError::from_token(
                name.clone(),
                "Already a variable with this name in scope.".into(),
            ));
        }

        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.lexeme.clone(), false);
        Ok(())
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.lexeme.clone(), true);
    }

    fn resolve_local(&mut self, name: &Token) {
        for (idx, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.borrow_mut().resolve(name, idx);
            }
        }
    }

    fn resolve_function(
        &mut self,
        params: &Vec<Token>,
        body: &Vec<Stmt>,
        function_type: FunctionType,
    ) -> AstResult<()> {
        let enclosing_body = self.current_function.clone();
        self.current_function = function_type;
        self.begin_scope();
        for param in params {
            self.declare(param)?;
            self.define(param);
        }
        self.resolve(body)?;
        self.end_scope();
        self.current_function = enclosing_body;
        Ok(())
    }
}

impl StmtVisitor for Resolver {
    fn visit_expression_stmt(&mut self, stmt: &crate::stmt::Stmt) -> AstResult<()> {
        let Stmt::Expression { expression } = stmt else {
            panic!("Expected variant.");
        };
        self.resolve_expr(expression)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &crate::stmt::Stmt) -> AstResult<()> {
        let Stmt::Print { expression } = stmt else {
            panic!("Expected variant.");
        };
        self.resolve_expr(expression)
    }

    fn visit_var_stmt(&mut self, stmt: &crate::stmt::Stmt) -> AstResult<()> {
        let Stmt::Var { name, expression } = stmt else {
            panic!("Expected variant.")
        };
        self.declare(name)?;
        if let Some(expr) = expression {
            self.resolve_expr(expr)?;
        }
        self.define(name);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &crate::stmt::Stmt) -> AstResult<()> {
        let Stmt::Block { statements } = stmt else {
            panic!("Expected block.")
        };
        self.begin_scope();
        self.resolve(statements)?;
        self.end_scope();
        Ok(())
    }

    fn visit_if_stmt(
        &mut self,
        expression: &crate::expr::Expr,
        if_stmt: &crate::stmt::Stmt,
        else_stmt: Option<&crate::stmt::Stmt>,
    ) -> AstResult<()> {
        self.resolve_expr(expression)?;
        self.resolve_stmt(if_stmt)?;
        if let Some(else_stmt) = else_stmt {
            self.resolve_stmt(else_stmt)?;
        }
        Ok(())
    }

    fn visit_while(
        &mut self,
        condition: &crate::expr::Expr,
        statement: &crate::stmt::Stmt,
    ) -> AstResult<()> {
        self.resolve_expr(condition)?;
        self.resolve_stmt(statement)?;
        Ok(())
    }

    fn visit_function(
        &mut self,
        name: &crate::token::Token,
        params: &Vec<crate::token::Token>,
        body: &Vec<crate::stmt::Stmt>,
    ) -> AstResult<()> {
        self.declare(name)?;
        self.define(name);

        self.resolve_function(params, body, FunctionType::Function)?;
        Ok(())
    }

    fn visit_return(
        &mut self,
        keyword: &crate::token::Token,
        value: &Option<crate::expr::Expr>,
    ) -> AstResult<()> {
        if let FunctionType::None = self.current_function {
            return Err(RuntimeError::from_token(
                keyword.clone(),
                "Cannot return from the top level.".into(),
            ));
        }
        if let Some(value) = value {
            if let FunctionType::Initializer = self.current_function {
                return Err(RuntimeError::from_token(
                    keyword.clone(),
                    "Can't return value from initializer.".into(),
                ));
            }
            self.resolve_expr(value)?;
        }
        Ok(())
    }

    fn visit_class(
        &mut self,
        name: &Token,
        _methods: &Vec<Stmt>,
        superclass: &Option<Token>,
    ) -> AstResult<()> {
        let class_type = self.enclosing_class.clone();
        self.enclosing_class = ClassType::Class;

        self.declare(name)?;
        self.define(name);

        if let Some(superclass) = superclass {
            self.enclosing_class = ClassType::Subclass;
            if superclass.lexeme == name.lexeme {
                return Err(RuntimeError::from_token(
                    name.clone(),
                    "A class can't inherit from itself.".into(),
                ));
            }
            self.resolve_expr(&Expr::Variable {
                name: superclass.clone(),
            })?;
        };
        if let Some(_) = superclass {
            self.begin_scope();
            self.scopes.last_mut().unwrap().insert("super".into(), true);
        };

        self.begin_scope();

        self.scopes.last_mut().unwrap().insert("this".into(), true);
        for method in _methods {
            let Stmt::Function {
                name,
                parameters,
                body,
            } = method
            else {
                panic!("Expected function.")
            };
            let declaration = if name.lexeme == "init" {
                FunctionType::Initializer
            } else {
                FunctionType::Method
            };
            self.resolve_function(parameters, body, declaration)?
        }
        self.end_scope();

        if let Some(_) = superclass {
            self.end_scope();
        }
        self.enclosing_class = class_type;
        Ok(())
    }
}

impl ExprVisitor<()> for Resolver {
    fn visit_binary(&mut self, expr: &crate::expr::Expr) -> AstResult<()> {
        let Expr::Binary {
            left,
            operator: _,
            right,
        } = expr
        else {
            panic!("Expected expression.")
        };
        self.resolve_expr(left)?;
        self.resolve_expr(right)?;
        Ok(())
    }

    fn visit_logical(
        &mut self,
        left: &crate::expr::Expr,
        _operator: &crate::token::Token,
        right: &crate::expr::Expr,
    ) -> AstResult<()> {
        self.resolve_expr(left)?;
        self.resolve_expr(right)?;
        Ok(())
    }

    fn visit_grouping(&mut self, expr: &crate::expr::Expr) -> AstResult<()> {
        let Expr::Grouping { expression } = expr else {
            panic!("Expected variant.");
        };
        self.resolve_expr(expression)
    }

    fn visit_literal(&mut self, _expr: &crate::expr::Expr) -> AstResult<()> {
        Ok(())
    }

    fn visit_unary(&mut self, expr: &crate::expr::Expr) -> AstResult<()> {
        let Expr::Unary { operator: _, right } = expr else {
            panic!("Expected variant.");
        };
        self.resolve_expr(right)?;
        Ok(())
    }

    fn visit_variable(&mut self, expr: &crate::expr::Expr) -> AstResult<()> {
        let Expr::Variable { name } = expr else {
            panic!("Expected variant.");
        };
        if let Some(last) = self.scopes.last() {
            if last.get(&name.lexeme) == Some(&false) {
                return Err(RuntimeError::from_token(
                    name.clone(),
                    "Can't read local variable in it's own initializer".into(),
                ));
            }
        }
        self.resolve_local(name);
        Ok(())
    }

    fn visit_assignment(&mut self, expr: &crate::expr::Expr) -> AstResult<()> {
        let Expr::Assignment { name, value } = expr else {
            panic!("Expected variant.")
        };
        self.resolve_expr(value)?;
        self.resolve_local(name);
        Ok(())
    }

    fn visit_call(
        &mut self,
        callee: &crate::expr::Expr,
        _paren: &crate::token::Token,
        arguments: &Vec<Box<crate::expr::Expr>>,
    ) -> AstResult<()> {
        self.resolve_expr(callee)?;
        for argument in arguments {
            self.resolve_expr(argument)?;
        }
        Ok(())
    }

    fn visit_get_expr(&mut self, object: &Expr, _name: &Token) -> AstResult<()> {
        self.resolve_expr(object)?;
        Ok(())
    }

    fn visit_set_expr(&mut self, object: &Expr, _name: &Token, value: &Expr) -> AstResult<()> {
        self.resolve_expr(object)?;
        self.resolve_expr(value)?;
        Ok(())
    }

    fn visit_this_expr(&mut self, keyword: &Token) -> AstResult<()> {
        if let ClassType::None = self.enclosing_class {
            return Err(RuntimeError::from_token(
                keyword.clone(),
                "Cannot use 'this' outside of a class.".into(),
            ));
        }
        self.resolve_local(keyword);
        Ok(())
    }

    fn visit_super_expr(&mut self, keyword: &Token, _method: &Token) -> AstResult<()> {
        match self.enclosing_class {
            ClassType::Subclass => {
                return Err(RuntimeError::from_token(
                    keyword.clone(),
                    "Can't use 'super' with no superclass.".into(),
                ));
            }
            ClassType::None => {
                return Err(RuntimeError::from_token(
                    keyword.clone(),
                    "Can't use super outside of a class.".into(),
                ));
            }
            _ => (),
        }
        self.resolve_local(keyword);
        Ok(())
    }
}
