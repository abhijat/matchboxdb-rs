use std::fmt;
use std::fmt::Formatter;

use crate::ast::select_statement::SelectStatement;
use crate::lexer::Token;

pub mod statement;
pub mod select_statement;

pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
pub struct IntLiteral {
    pub token: Token,
    pub value: i64,
}

impl fmt::Display for IntLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub token: Token,
    // TODO use enum
    pub operator: String,
    pub right: Box<Expression>,
}

impl fmt::Display for PrefixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    Int(IntLiteral),
    Prefixed(PrefixExpression),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(expression) => fmt::Display::fmt(&expression, f),
            Expression::Int(int_literal) => fmt::Display::fmt(&int_literal, f),
            Expression::Prefixed(prefix_expression) => fmt::Display::fmt(&prefix_expression, f),
        }
    }
}