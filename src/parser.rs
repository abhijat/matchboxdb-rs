use crate::ast::{Expression, Identifier};
use crate::ast::select_statement::SelectStatement;
use crate::ast::statement::Statement;
use crate::lexer::{Lexer, Token, TokenKind};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

// Parsing functions
impl Parser {
    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.kind {
            TokenKind::Select => self.parse_select_statement(),
            _ => unimplemented!()
        }
    }

    fn parse_select_statement(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();

        let expressions = self.parse_expression_list()?;
        if !self.expect_peek(TokenKind::From) {
            return None;
        }

        let table_name = self.parse_table_name()?;
        Some(Statement::Select(SelectStatement::new(token, table_name, expressions)))
    }

    fn parse_expression_list(&mut self) -> Option<Vec<Expression>> {
        let mut expressions = vec![];

        // At least one identifier is required - move to it
        if !self.expect_peek(TokenKind::Identifier) {
            return None;
        }

        let identifier = self.parse_identifier();
        expressions.push(identifier);

        // keep collecting expressions until we run out of them
        while self.peek_token_is(TokenKind::Comma) {
            // make comma the current token
            self.next_token();

            // an identifier should follow
            if !self.expect_peek(TokenKind::Identifier) {
                return None;
            }

            expressions.push(self.parse_identifier());
        }

        Some(expressions)
    }

    fn parse_table_name(&mut self) -> Option<Expression> {
        if !self.expect_peek(TokenKind::Identifier) {
            return None;
        }

        Some(self.parse_identifier())
    }

    fn parse_identifier(&mut self) -> Expression {
        Expression::Identifier(Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        })
    }
}

// Helpers
impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser { lexer, current_token, peek_token, errors: vec![] }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn current_token_is(&self, kind: TokenKind) -> bool {
        self.current_token.kind == kind
    }

    fn peek_token_is(&self, kind: TokenKind) -> bool {
        self.peek_token.kind == kind
    }

    fn expect_peek(&mut self, kind: TokenKind) -> bool {
        if self.peek_token_is(kind) {
            self.next_token();
            true
        } else {
            self.errors.push(format!("expected {:?}, found {:?}", kind, self.peek_token.kind));
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> Statement {
        let mut parser = Parser::new(Lexer::new(s));
        let statement = parser.parse_statement();
        assert!(parser.errors.is_empty());
        statement.unwrap()
    }

    fn assert_identifier(s: &str, expression: &Expression) {
        match expression {
            Expression::Identifier(Identifier { value, .. }) => assert_eq!(value, s),
        }
    }

    fn assert_token(token: &Token, kind: TokenKind, literal: &str) {
        assert_eq!(token.kind, kind);
        assert_eq!(token.literal, literal);
    }

    fn assert_select_statement(statement: &Statement, table_name: &str, attributes: &[&str]) {
        if let Statement::Select(s) = statement {
            assert_token(&s.token, TokenKind::Select, "SELECT");
            assert_identifier(table_name, &s.table_name);
            for (expect, got) in attributes.iter().zip(s.expressions.iter()) {
                assert_identifier(expect, got);
            }
        } else {
            panic!("{:?} is not a select statement", statement)
        }
    }

    #[test]
    fn parse_select_statement() {
        struct Test<'a> {
            input: &'a str,
            table_name: &'a str,
            expressions: &'a [&'a str],
        }

        for t in &[
            Test { input: "select name, age, gender from employee", table_name: "employee", expressions: &["name", "age", "gender"] },
            Test { input: "select gender from foobar", table_name: "foobar", expressions: &["gender"] },
        ] {
            let statement = parse(t.input);
            println!("{}", statement);
            assert_select_statement(&statement, t.table_name, t.expressions);
        }
    }

    #[test]
    fn parse_bad_select_statement() {
        for (input, expected_error) in &[
            ("select foo from", "expected Identifier, found Eof"),
            ("select blah", "expected From, found Eof"),
            ("select from bar", "expected Identifier, found From"),
        ] {
            let mut p = Parser::new(Lexer::new(input));
            let statement = p.parse_statement();
            assert!(statement.is_none());
            let error = &p.errors[0];
            assert_eq!(error, expected_error);
        }
    }

    #[test]
    fn parse_stringify_select() {
        let statement = parse("select name, age, gender from employee");
        assert_eq!("SELECT name, age, gender FROM employee", format!("{}", statement));
    }
}