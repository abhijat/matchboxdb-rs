use std::fmt;
use std::fmt::{Debug, Formatter};

use crate::ast::{Expression, Node};
use crate::ast::select_statement::SelectStatement;
use crate::lexer::Token;

#[derive(Debug)]
pub enum Statement {
    Select(SelectStatement),
    Expr(ExpressionStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Select(select_statement) => select_statement.token.literal.clone(),
            Statement::Expr(expression) => expression.token.literal.clone(),
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Select(select) => fmt::Display::fmt(&select, f),
            Statement::Expr(expression) => fmt::Display::fmt(&expression, f),
        }
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Expression,
}

impl fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expression)
    }
}