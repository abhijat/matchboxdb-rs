use std::fmt;
use std::fmt::Formatter;

use expression::Expression;

use crate::lexer::Token;

pub mod statement;
pub mod select_statement;
pub mod expression;
pub mod identifier;
pub mod int_literal;
pub mod prefix_expression;

pub trait Node {
    fn token_literal(&self) -> String;
}

