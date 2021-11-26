use std::fmt;
use std::fmt::Formatter;

use crate::ast::expression::Expression;
use crate::lexer::Token;

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
