use std::fmt;
use std::fmt::Formatter;

use crate::ast::identifier::Identifier;
use crate::ast::int_literal::IntLiteral;
use crate::ast::prefix_expression::PrefixExpression;

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
