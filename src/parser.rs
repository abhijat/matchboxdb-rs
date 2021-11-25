use std::collections::HashMap;

use crate::ast::{Expression, Identifier, IntLiteral};
use crate::ast::select_statement::SelectStatement;
use crate::ast::statement::{ExpressionStatement, Statement};
use crate::lexer::{Lexer, Token, TokenKind};

enum Precedence {
    Lowest = 1,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
}

type PrefixParser = fn(&mut Parser) -> Expression;
type InfixParser = fn(&mut Parser, Expression) -> Expression;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
    prefix_parsers: HashMap<TokenKind, PrefixParser>,
    infix_parsers: HashMap<TokenKind, InfixParser>,
}

// Parsing functions
impl Parser {
    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.kind {
            TokenKind::Select => self.parse_select_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();
        let expression = self.parse_expression(Precedence::Lowest)?;
        let statement = Statement::Expr(ExpressionStatement { token, expression });
        Some(statement)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let prefix = self.prefix_parsers.get(&self.current_token.kind)?;
        Some(prefix(self))
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

    fn parse_integer_literal(&mut self) -> Expression {
        Expression::Int(IntLiteral {
            token: self.current_token.clone(),
            value: self.current_token.literal.parse().unwrap(),
        })
    }
}

// Initializers
impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        let mut p = Parser {
            lexer,
            current_token,
            peek_token,
            errors: vec![],
            prefix_parsers: Default::default(),
            infix_parsers: Default::default(),
        };

        p.register_prefix(TokenKind::Identifier, Parser::parse_identifier);
        p.register_prefix(TokenKind::Int, Parser::parse_integer_literal);
        p
    }

    fn register_prefix(&mut self, kind: TokenKind, prefix_parser: PrefixParser) {
        self.prefix_parsers.insert(kind, prefix_parser);
    }

    fn register_infix(&mut self, kind: TokenKind, infix_parser: InfixParser) {
        self.infix_parsers.insert(kind, infix_parser);
    }
}

// Token Helpers
impl Parser {
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
            _ => panic!("{} is not an identifier", expression)
        }
    }

    fn assert_int_literal(value: i64, expression: &Expression) {
        if let Expression::Int(i) = expression {
            assert_eq!(i.value, value);
        } else {
            panic!("{} is not an int literal", expression)
        }
    }

    fn assert_expression<T>(t: T, expression: &Expression) {}

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

    #[test]
    fn parse_identifier() {
        let statement = parse("somename;");
        let expected = String::from("somename");
        assert!(matches!(statement, Statement::Expr(ExpressionStatement {
            expression: Expression::Identifier(Identifier {
                value: expected, ..
            }), ..
        })));
    }

    #[test]
    fn parse_int() {
        let statement = parse("1234");
        assert!(matches!(statement, Statement::Expr(ExpressionStatement {
            expression: Expression::Int(IntLiteral {
                value: 1234, ..
            }), ..
        })));
    }
}