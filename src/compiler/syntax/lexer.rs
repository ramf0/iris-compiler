use std::iter::Peekable;
use std::process::exit;
use std::str::Chars;
use crate::compiler::exception::Exception;
use crate::compiler::syntax::token::{Token, TokenValue};

pub struct Lexer<'a> {
    src: &'a str,
    chars: Peekable<Chars<'a>>,
    start: usize,
    pos: usize,
    pub column: usize,
    pub line: usize,
    current: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut chars = src.chars().peekable();
        let current = chars.next();
        Lexer {
            src,
            chars: chars,
            start: 0,
            pos: 0,
            column: 0,
            line: 1,
            current,
        }
    }

    fn next_char(&mut self) {
        if self.pos == self.src.len() - 1 { self.current = Some('\0'); }
        else {
            self.pos += 1;
            self.column += 1;
            self.current = self.chars.next();
        }
    }

    fn word_tokenize(&mut self) -> Token<'a> {
        self.start = self.pos;
        while let Some(c) = self.current {
            if c.is_alphanumeric() {
                self.next_char();
            } else {
                break;
            }
        }

        self.check_keyword()
    }

    fn check_keyword(&mut self) -> Token<'a> {
        let word = &self.src[self.start..self.pos];

        match word {
            _ => Token::new(TokenValue::Identifier(word), self.line, self.column),
        }
    }

    fn string_tokenize(&mut self) -> Token<'a> {
        self.next_char();
        self.start = self.pos;
        while let Some(c) = self.current {
            if c == '\'' {
                break;
            } else {
                self.next_char();
            }
        }
        if self.current.is_none() {
            Exception::new().throw("Compile:Lexical",
            format!("unterminated string: {}", &self.src[self.start..self.pos]).as_str(), "file", self.line, self.column);
            exit(1);
        } else {
            let string = &self.src[self.start..self.pos];
            self.next_char();
            Token::new(TokenValue::StringLiteral(string), self.line, self.column)
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current {
            if c == '\n' {
                self.column = 0;
                self.line += 1;
            } else if c.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let token = match self.current {
            Some('(') => Token::new(TokenValue::LParen, self.line, self.column),
            Some(')') => Token::new(TokenValue::RParen, self.line, self.column),
            Some('\0') => Token::new(TokenValue::Eof, self.line, self.column),

            Some('\'') => return Some(self.string_tokenize()),

            _ => {
                if let Some(c) = self.current {
                    match c {
                        'a'..='z' | 'A'..='Z' | '_' => return Some(self.word_tokenize()),
                        _ => {
                            Exception::new().throw("Compile:Lexical",
                            format!("unknown character '{:?}'c", self.current).as_str(), "file", self.line, self.column);
                            return None;
                        },
                    }
                } else {
                    return None
                }
            }
        };

        self.next_char();
        Some(token)
    }
}