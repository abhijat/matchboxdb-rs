use std::fmt;
use std::fmt::Formatter;

use crate::ast::expression::Expression;
use crate::ast::Node;
use crate::lexer::Token;

#[derive(Debug)]
pub struct SelectStatement {
    pub token: Token,
    pub table_name: Expression,
    pub expressions: Vec<Expression>,
}

impl SelectStatement {
    pub fn new(token: Token, table_name: Expression, expressions: Vec<Expression>) -> Self {
        SelectStatement { token, table_name, expressions }
    }
}

impl fmt::Display for SelectStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.token.literal)?;

        let expressions = self.expressions
            .iter()
            .map(|expression| format!("{}", expression))
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "{} FROM {}", expressions, self.table_name)
    }
}
