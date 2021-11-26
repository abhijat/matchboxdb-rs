use std::fmt;
use std::fmt::Formatter;

use crate::lexer::Token;

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
