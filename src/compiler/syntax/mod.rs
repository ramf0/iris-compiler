use std::process::exit;
use crate::compiler::ast::expressions::Expression::StringLiteral;
use crate::compiler::ast::statements::Statement;
use crate::compiler::ast::statements::Statement::CallFunction;
use crate::compiler::syntax::lexer::Lexer;
use crate::compiler::syntax::token::{Token, TokenValue};
use crate::compiler::exception::Exception;

mod token;
mod lexer;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer: Lexer<'a> = Lexer::new(source);
        let current = lexer.next();
        if let None = &current {
            Exception::new().throw("Compile:Syntax",
            "current token is None", "file", lexer.line, lexer.column);
            exit(0);
        }
        Parser {
            lexer: lexer,
            current_token: current.unwrap(),
        }
    }

    fn parse_call_expression(&mut self) -> Option<Statement<'a>> {
        if let TokenValue::Identifier(name) = self.current_token.value {
            self.match_token(TokenValue::Identifier(""));
            self.match_token(TokenValue::LParen);
            let value = self.match_token(TokenValue::StringLiteral(""));
            self.match_token(TokenValue::RParen);
            return Some(CallFunction {
                name: name,
                args: vec![StringLiteral(value)],
            })
        }
        None
    }

    fn match_token(&mut self, token: TokenValue) -> &'a str {
        match (&self.current_token.value.clone(), &token) {
            (TokenValue::StringLiteral(value), TokenValue::StringLiteral(_)) => {
                self.next_token();
                value
            }
            (TokenValue::Identifier(value), TokenValue::Identifier(_)) => {
                self.next_token();
                value
            }
            _ => {
                if self.current_token.value == token {
                    self.next_token();
                    ""
                } else {
                    Exception::new().throw("Compile:Syntax",
                    format!("token {:?} is break the syntax; expect {:?}",
                            self.current_token, token).as_str(), "file", self.lexer.line, self.lexer.column);
                    exit(0);
                }
            }
        }
    }

    fn next_token(&mut self) {
        let cur = self.lexer.next();
        let current_token = self.check_token(cur);
        self.current_token = current_token;
    }

    fn check_token(&mut self, token: Option<Token<'a>>) -> Token<'a> {
        if let None = token {
            Exception::new().throw("Compile:Syntax",
                                   "current token is None", "file", self.lexer.line, self.lexer.column);
            exit(0);
        } else {
            token.unwrap()
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Statement<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_call_expression()
    }
}