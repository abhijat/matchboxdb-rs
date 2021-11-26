use std::collections::HashMap;

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone, Eq, Hash)]
pub enum TokenKind {
    Illegal,
    Eof,
    Identifier,
    Int,
    Equals,
    NotEq,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Select,
    Update,
    Insert,
    Delete,
    From,
    Table,
    Where,
    Values,
    Into,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

pub fn lookup_identifier(s: &str) -> TokenKind {
    let keywords: HashMap<&str, TokenKind> = HashMap::from([
        ("SELECT", TokenKind::Select),
        ("INSERT", TokenKind::Insert),
        ("UPDATE", TokenKind::Update),
        ("DELETE", TokenKind::Delete),
        ("FROM", TokenKind::From),
        ("TABLE", TokenKind::Table),
        ("WHERE", TokenKind::Where),
        ("VALUES", TokenKind::Values),
        ("INTO", TokenKind::Into),
    ]);

    keywords.get(s.to_uppercase().as_str())
        .or(Some(&TokenKind::Identifier))
        .unwrap()
        .clone()
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: None,
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = Some('\0')
        } else {
            self.ch = self.input.chars().nth(self.read_position);
        }
        self.position = self.read_position;
        self.read_position += 1
    }

    pub fn next_token(&mut self) -> Token {
        let token: Token;

        self.eat_ws();

        match self.ch {
            Some('=') => {
                token = Token { kind: TokenKind::Equals, literal: "=".to_string() };
            }
            Some(';') => {
                token = Token { kind: TokenKind::Semicolon, literal: ";".to_string() };
            }
            Some(',') => {
                token = Token { kind: TokenKind::Comma, literal: ",".to_string() };
            }
            Some('(') => {
                token = Token { kind: TokenKind::LParen, literal: "(".to_string() };
            }
            Some(')') => {
                token = Token { kind: TokenKind::RParen, literal: ")".to_string() };
            }
            Some('{') => {
                token = Token { kind: TokenKind::LBrace, literal: "{".to_string() };
            }
            Some('}') => {
                token = Token { kind: TokenKind::RBrace, literal: "}".to_string() };
            }
            Some('+') => {
                token = Token { kind: TokenKind::Plus, literal: "+".to_string() };
            }
            Some('-') => {
                token = Token { kind: TokenKind::Minus, literal: "-".to_string() };
            }
            Some('!') => {
                let peek = self.peek_char();
                token = if let Some('=') = peek {
                    self.read_char();
                    Token { kind: TokenKind::NotEq, literal: "!=".to_string() }
                } else {
                    Token { kind: TokenKind::Bang, literal: "!".to_string() }
                };
            }
            Some('*') => {
                token = Token { kind: TokenKind::Asterisk, literal: "*".to_string() };
            }
            Some('/') => {
                token = Token { kind: TokenKind::Slash, literal: "/".to_string() };
            }
            Some('<') => {
                token = Token { kind: TokenKind::Lt, literal: "<".to_string() };
            }
            Some('>') => {
                token = Token { kind: TokenKind::Gt, literal: ">".to_string() };
            }
            Some('\0') => {
                token = Token { kind: TokenKind::Eof, literal: "\0".to_string() };
            }
            Some(t) => {
                if t.is_alphabetic() {
                    let literal = self.read_identifier();
                    let kind = lookup_identifier(&literal);
                    let literal = if kind == TokenKind::Identifier {
                        literal
                    } else {
                        literal.to_uppercase()
                    };

                    return Token { kind, literal };

                } else if t.is_digit(10) {
                    let literal = self.read_number();
                    return Token { kind: TokenKind::Int, literal };
                } else {
                    token = Token { kind: TokenKind::Illegal, literal: "".to_string() };
                }
            }
            None => {
                return Token { kind: TokenKind::Illegal, literal: "".to_string() };
            }
        }

        self.read_char();
        return token;
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = vec![];

        while self.ch.unwrap().is_alphanumeric() {
            identifier.push(self.ch.unwrap());
            self.read_char();
        }

        identifier.into_iter().collect()
    }

    fn eat_ws(&mut self) {
        while self.ch.unwrap().is_whitespace() {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> String {
        let mut number = vec![];
        while self.ch.unwrap().is_digit(10) {
            number.push(self.ch.unwrap());
            self.read_char();
        }

        number.into_iter().collect()
    }

    fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            self.input.chars().nth(self.read_position)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_token() {
        let input = r#"=+(){},;
        SELECT UPDATE INSERT DELETE 12 23      1234

        1 + 2 = 1000 1 > < !12 FROM TABLE into Where values != = abcd, 1122!"#;
        let mut lexer = Lexer::new(input);
        let tests = vec![
            Token { kind: TokenKind::Equals, literal: "=".to_string() },
            Token { kind: TokenKind::Plus, literal: "+".to_string() },
            Token { kind: TokenKind::LParen, literal: "(".to_string() },
            Token { kind: TokenKind::RParen, literal: ")".to_string() },
            Token { kind: TokenKind::LBrace, literal: "{".to_string() },
            Token { kind: TokenKind::RBrace, literal: "}".to_string() },
            Token { kind: TokenKind::Comma, literal: ",".to_string() },
            Token { kind: TokenKind::Semicolon, literal: ";".to_string() },
            Token { kind: TokenKind::Select, literal: "SELECT".to_string() },
            Token { kind: TokenKind::Update, literal: "UPDATE".to_string() },
            Token { kind: TokenKind::Insert, literal: "INSERT".to_string() },
            Token { kind: TokenKind::Delete, literal: "DELETE".to_string() },
            Token { kind: TokenKind::Int, literal: "12".to_string() },
            Token { kind: TokenKind::Int, literal: "23".to_string() },
            Token { kind: TokenKind::Int, literal: "1234".to_string() },
            Token { kind: TokenKind::Int, literal: "1".to_string() },
            Token { kind: TokenKind::Plus, literal: "+".to_string() },
            Token { kind: TokenKind::Int, literal: "2".to_string() },
            Token { kind: TokenKind::Equals, literal: "=".to_string() },
            Token { kind: TokenKind::Int, literal: "1000".to_string() },
            Token { kind: TokenKind::Int, literal: "1".to_string() },
            Token { kind: TokenKind::Gt, literal: ">".to_string() },
            Token { kind: TokenKind::Lt, literal: "<".to_string() },
            Token { kind: TokenKind::Bang, literal: "!".to_string() },
            Token { kind: TokenKind::Int, literal: "12".to_string() },
            Token { kind: TokenKind::From, literal: "FROM".to_string() },
            Token { kind: TokenKind::Table, literal: "TABLE".to_string() },
            Token { kind: TokenKind::Into, literal: "INTO".to_string() },
            Token { kind: TokenKind::Where, literal: "WHERE".to_string() },
            Token { kind: TokenKind::Values, literal: "VALUES".to_string() },
            Token { kind: TokenKind::NotEq, literal: "!=".to_string() },
            Token { kind: TokenKind::Equals, literal: "=".to_string() },
            Token { kind: TokenKind::Identifier, literal: "abcd".to_string() },
            Token { kind: TokenKind::Comma, literal: ",".to_string() },
            Token { kind: TokenKind::Int, literal: "1122".to_string() },
            Token { kind: TokenKind::Bang, literal: "!".to_string() },
        ];

        for test in tests {
            let token = lexer.next_token();
            assert_eq!(token.kind, test.kind, "failed kind check, found {:?}, expected {:?}", token.kind, test.kind);
            assert_eq!(token.literal, test.literal, "failed literal check, found {:?}, expected {:?}", token.literal, test.literal);
        }
    }
}
