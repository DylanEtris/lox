//! The definition for the Lox tokens
use std::{
    fmt::{Debug, Display},
    ops,
    rc::Rc,
};

use crate::lox_callable::LoxCallable;

#[derive(Clone)]
pub enum Literal {
    String { val: String },
    Number { val: f64 },
    Bool { val: bool },
    Callable(Rc<dyn LoxCallable>),
    Nil,
}

impl Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String { val } => f.debug_struct("String").field("val", val).finish(),
            Self::Number { val } => f.debug_struct("Number").field("val", val).finish(),
            Self::Bool { val } => f.debug_struct("Bool").field("val", val).finish(),
            Self::Callable(_) => panic!("Cannot debug a callable"),
            Self::Nil => write!(f, "Nil"),
        }
    }
}

impl Literal {
    fn cast_f64(self) -> Result<f64, String> {
        match self {
            Self::Number { val } => Ok(val),
            _ => Err("Expected a number.".to_string()),
        }
    }
}

impl ops::Not for Literal {
    type Output = Literal;

    fn not(self) -> Self::Output {
        match self {
            Self::Bool { val } => Self::Bool { val: !val },
            _ => panic!("Expected a bool."),
        }
    }
}

impl ops::Add for Literal {
    type Output = Literal;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number { val: left }, Self::Number { val: right }) => {
                Literal::Number { val: left + right }
            }
            (Self::String { val: left }, Self::String { val: right }) => {
                Literal::String { val: left + &right }
            }
            _ => Literal::Nil,
        }
    }
}

impl ops::Div for Literal {
    type Output = Literal;

    fn div(self, rhs: Self) -> Self::Output {
        let left = self.cast_f64().unwrap();
        let right = rhs.cast_f64().unwrap();
        Literal::Number { val: left / right }
    }
}

impl ops::Mul for Literal {
    type Output = Literal;

    fn mul(self, rhs: Self) -> Self::Output {
        let left = self.cast_f64().unwrap();
        let right = rhs.cast_f64().unwrap();
        Literal::Number { val: left * right }
    }
}

impl ops::Neg for Literal {
    type Output = Literal;

    fn neg(self) -> Self::Output {
        let val = self.cast_f64().unwrap();
        Literal::Number { val: -val }
    }
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Number { val: left }, Self::Number { val: right }) => left.partial_cmp(right),
            _ => panic!("Expected number."),
        }
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String { val: l_val }, Self::String { val: r_val }) => l_val == r_val,
            (Self::Number { val: l_val }, Self::Number { val: r_val }) => l_val == r_val,
            (Self::Bool { val: l_val }, Self::Bool { val: r_val }) => l_val == r_val,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl ops::Sub for Literal {
    type Output = Literal;

    fn sub(self, rhs: Self) -> Self::Output {
        let left = self.cast_f64().unwrap();
        let right = rhs.cast_f64().unwrap();
        Literal::Number { val: left - right }
    }
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        match self {
            Literal::String { val } => val.to_string(),
            Literal::Number { val } => val.to_string(),
            Literal::Bool { val } => val.to_string(),
            Literal::Nil => "nil".to_string(),
            Literal::Callable(_) => "Callable".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

/// Lox token
#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}
