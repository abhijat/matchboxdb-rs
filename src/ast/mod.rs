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
pub enum Expression {
    Identifier(Identifier)
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(expression) => fmt::Display::fmt(&expression, f),
        }
    }
}