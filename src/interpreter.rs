use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    time::{self, SystemTime},
};

use crate::{
    environment::Environment,
    error::{AstResult, RuntimeError},
    expr::{Expr, ExprVisitor},
    lox_callable::LoxCallable,
    lox_class::LoxClass,
    lox_function::LoxFunction,
    lox_instance::instance_get,
    stmt::{Stmt, StmtVisitor},
    token::{Literal, Token, TokenType},
};

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    locals: HashMap<Token, usize>,
    environment: Rc<RefCell<Environment>>,
}

impl Default for Interpreter {
    fn default() -> Self {
        struct Clock {}

        impl LoxCallable for Clock {
            fn call(
                &self,
                _interpreter: &mut Interpreter,
                _arguments: Vec<Literal>,
            ) -> AstResult<Literal> {
                let current_time = time::SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap();
                return Ok(Literal::Number {
                    val: current_time.as_secs_f64(),
                });
            }

            fn arity(&self) -> usize {
                return 0;
            }
        }
        let globals = Rc::new(RefCell::new(Environment::default()));
        globals
            .borrow_mut()
            .define("clock".into(), Literal::Callable(Rc::new(Clock {})));
        Self {
            environment: Rc::clone(&globals),
            globals: globals,
            locals: HashMap::default(),
        }
    }
}

impl Interpreter {
    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> AstResult<()> {
        for stmt in stmts {
            self.execute(&stmt)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> AstResult<()> {
        stmt.accept(self)
    }

    fn evaluate(&mut self, expr: &Expr) -> AstResult<Literal> {
        expr.accept(self)
    }

    fn is_truthy(&self, value: &Literal) -> bool {
        match value {
            Literal::Nil => false,
            Literal::Bool { val } => *val,
            _ => true,
        }
    }

    fn check_number_operand(&self, operator: Token, value: &Literal) -> AstResult<()> {
        let val = match value {
            Literal::Number { val: _ } => Ok(()),
            _ => Err(RuntimeError::from_token(
                operator,
                "Operand must be a number.".to_string(),
            )),
        };
        return val;
    }

    fn check_number_operands(
        &self,
        operator: Token,
        left: &Literal,
        right: &Literal,
    ) -> AstResult<()> {
        match (left, right) {
            (Literal::Number { val: _ }, Literal::Number { val: _ }) => Ok(()),
            _ => Err(RuntimeError::from_token(
                operator.clone(),
                format!("Operand for '{}' must be a number.", &operator.lexeme),
            )),
        }
    }

    fn check_number_or_string_operands(
        &self,
        operator: Token,
        left: &Literal,
        right: &Literal,
    ) -> AstResult<()> {
        match (left, right) {
            (Literal::Number { val: _ }, Literal::Number { val: _ }) => Ok(()),
            (Literal::String { val: _ }, Literal::String { val: _ }) => Ok(()),
            _ => Err(RuntimeError::from_token(
                operator.clone(),
                format!(
                    "Operand for '{}' must be a number or a string.\nGot {:?} and {:?}.",
                    operator.lexeme, left, right
                ),
            )),
        }
    }

    pub fn execute_block(
        &mut self,
        statements: &Vec<Stmt>,
        environment: Environment,
    ) -> AstResult<()> {
        let previous = Rc::clone(&self.environment);
        self.environment = Rc::new(RefCell::new(environment));
        let mut result = Ok(());
        for statement in statements {
            result = self.execute(statement);
            if let Err(_) = result {
                break;
            }
        }

        self.environment = previous;
        result
    }

    pub(crate) fn resolve(&mut self, name: &Token, depth: usize) {
        self.locals.insert(name.clone(), depth);
    }

    fn look_up_variable(&self, name: &Token) -> Result<Literal, RuntimeError> {
        match self.locals.get(name) {
            Some(distance) => self.environment.borrow().get_at(*distance, &name.lexeme),
            None => self.globals.borrow().get(&name.lexeme),
        }
    }
}

impl ExprVisitor<Literal> for Interpreter {
    fn visit_binary(&mut self, expr: &Expr) -> AstResult<Literal> {
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

    fn visit_grouping(&mut self, expr: &Expr) -> AstResult<Literal> {
        let Expr::Grouping { expression } = expr else {
            panic!("Expected variant.")
        };
        self.evaluate(expression)
    }

    fn visit_literal(&mut self, expr: &Expr) -> AstResult<Literal> {
        let Expr::Literal { value } = expr else {
            panic!("Expected variant.")
        };
        Ok(value.clone())
    }

    fn visit_unary(&mut self, expr: &Expr) -> AstResult<Literal> {
        let Expr::Unary { operator, right } = expr else {
            panic!("Expected variant.")
        };
        let value = self.evaluate(right)?;
        match operator.token_type {
            TokenType::Bang => {
                return Ok(Literal::Bool {
                    val: self.is_truthy(&value),
                });
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

    fn visit_variable(&mut self, expr: &Expr) -> AstResult<Literal> {
        let Expr::Variable { name } = expr else {
            panic!("Expected variant.")
        };
        self.look_up_variable(name)
    }

    fn visit_assignment(&mut self, expr: &Expr) -> AstResult<Literal> {
        let Expr::Assignment { name, value } = expr else {
            panic!("Expected variant.")
        };
        let value = self.evaluate(value)?;
        match self.locals.get(name) {
            Some(distance) => self
                .environment
                .borrow_mut()
                .assign_at(*distance, name, &value)?,
            None => self.globals.borrow_mut().assign(name, &value)?,
        };
        Ok(value)
    }

    fn visit_logical(&mut self, left: &Expr, operator: &Token, right: &Expr) -> AstResult<Literal> {
        match operator.token_type {
            TokenType::And => {
                let left = self.evaluate(left)?;
                if !self.is_truthy(&left) {
                    return Ok(left);
                }
            }
            TokenType::Or => {
                let left = self.evaluate(left)?;
                if self.is_truthy(&left) {
                    return Ok(left);
                }
            }
            _ => panic!("Expected an 'and' or an 'or'"),
        }
        self.evaluate(&right)
    }

    fn visit_call(
        &mut self,
        callee: &Expr,
        paren: &Token,
        arguments: &Vec<Box<Expr>>,
    ) -> AstResult<Literal> {
        let callee = self.evaluate(callee)?;
        let arguments: Result<Vec<Literal>, _> =
            arguments.iter().map(|arg| self.evaluate(arg)).collect();
        let arguments = arguments?;
        let function = match callee {
            Literal::Callable(x) => x,
            Literal::Class(klass) => Rc::new(klass),
            _ => {
                return Err(RuntimeError::from_token(
                    paren.clone(),
                    "Can only call functions and classes.".into(),
                ));
            }
        };
        if arguments.len() != function.arity() {
            return Err(RuntimeError::from_token(
                paren.clone(),
                format!(
                    "Expected {} arguments but got {}.",
                    function.arity(),
                    arguments.len()
                ),
            ));
        }
        function.call(self, arguments)
    }

    fn visit_get_expr(&mut self, object: &Expr, name: &Token) -> AstResult<Literal> {
        match self.evaluate(object)? {
            Literal::Instance(object) => instance_get(&object, name),
            _ => Err(RuntimeError::from_token(
                name.clone(),
                "Only instances have properties.".into(),
            )),
        }
    }

    fn visit_set_expr(&mut self, object: &Expr, name: &Token, value: &Expr) -> AstResult<Literal> {
        match self.evaluate(object)? {
            Literal::Instance(instance) => {
                let value = self.evaluate(value)?;
                instance.borrow_mut().set(name, value.clone())?;
                Ok(value)
            }
            _ => Err(RuntimeError::from_token(
                name.clone(),
                "Only instances have fields.".into(),
            )),
        }
    }

    fn visit_this_expr(&mut self, keyword: &Token) -> AstResult<Literal> {
        self.look_up_variable(keyword)
    }
}

impl StmtVisitor for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> AstResult<()> {
        let Stmt::Expression { expression } = stmt else {
            panic!("Expected variant.")
        };
        self.evaluate(expression)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) -> AstResult<()> {
        let Stmt::Print { expression } = stmt else {
            panic!("Expected variant.")
        };
        let value = self.evaluate(expression)?;
        println!("{}", value.to_string());
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) -> AstResult<()> {
        let Stmt::Var { name, expression } = stmt else {
            panic!("Expected variant.")
        };
        let value = match expression {
            Some(expr) => self.evaluate(expr)?,
            None => Literal::Nil,
        };
        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &Stmt) -> AstResult<()> {
        let Stmt::Block { statements } = stmt else {
            panic!("Expected variant.")
        };
        let environment = Environment::new(Rc::clone(&self.environment));
        self.execute_block(statements, environment)
    }

    fn visit_if_stmt(
        &mut self,
        expression: &Expr,
        if_stmt: &Stmt,
        else_stmt: Option<&Stmt>,
    ) -> AstResult<()> {
        let condition = self.evaluate(expression)?;
        if self.is_truthy(&condition) {
            self.execute(if_stmt)?;
        } else if let Some(else_stmt) = else_stmt {
            self.execute(else_stmt)?;
        }
        Ok(())
    }

    fn visit_while(&mut self, condition: &Expr, statement: &Stmt) -> AstResult<()> {
        let mut guard = self.evaluate(condition)?;
        while self.is_truthy(&guard) {
            self.execute(statement)?;
            guard = self.evaluate(condition)?;
        }
        Ok(())
    }

    fn visit_function(
        &mut self,
        name: &Token,
        params: &Vec<Token>,
        body: &Vec<Stmt>,
    ) -> AstResult<()> {
        let value = LoxFunction {
            name: name.clone(),
            parameters: params.to_vec(),
            body: body.to_vec(),
            closure: Rc::clone(&self.environment),
            is_initializer: false,
        };
        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), Literal::Callable(Rc::new(value)));
        Ok(())
    }

    fn visit_return(&mut self, _keyword: &Token, value: &Option<Expr>) -> AstResult<()> {
        let return_val = match value {
            Some(expr) => self.evaluate(expr)?,
            None => Literal::Nil,
        };
        return Err(RuntimeError::Return(return_val));
    }

    fn visit_class(&mut self, name: &Token, methods: &Vec<Stmt>) -> AstResult<()> {
        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), Literal::Nil);
        let mut function_methods = HashMap::new();
        for method in methods {
            let Stmt::Function {
                name,
                parameters,
                body,
            } = method
            else {
                panic!("Expected function statement.")
            };
            function_methods.insert(
                name.lexeme.clone(),
                Rc::new(LoxFunction {
                    name: name.clone(),
                    parameters: parameters.to_vec(),
                    body: body.to_vec(),
                    closure: Rc::clone(&self.environment),
                    is_initializer: name.lexeme == "init",
                }),
            );
        }
        let klass = Literal::Class(LoxClass::new(name.lexeme.clone(), function_methods));
        self.environment.borrow_mut().assign(name, &klass)?;
        Ok(())
    }
}
