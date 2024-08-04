#![allow(dead_code)]
#![allow(unused)]

use std::thread::current;

pub enum Token {
    Import(ImportType),
    Struct(String),
    Identifier,
    LParen,
    RParen,
    LCurly,
    RCurly,
    Quotation,
    Exclam,
    Question,
    Colon,
    Semicolon,
    Space,
    EOF,
}

pub enum ImportType {
    File,
    Lib,
}

pub struct Lexer {
    tokens: Vec<Token>,
    source: String,
    current: usize,
}

impl Lexer {
    pub fn from(source: String) -> Self {
        Lexer {
            tokens: Vec::new(),
            source,
            current: 0,
        }
    }
}

impl Lexer {
    pub fn lex(&mut self) {
        loop {
            println!("Current {}", self.current() as char);

            if let None = self.consume(Token::EOF) {
                break;
            }
        }
    }

    fn current(&self) -> u8 {
        self.source.as_bytes()[self.current]
    }

    fn next(&mut self) -> Option<u8> {
        if self.current + 1 >= self.source.len() {
            return None;
        }

        self.current += 1;

        Some(self.source.as_bytes()[self.current])
    }

    fn peek(&self) -> Option<u8> {
        if self.current + 1 >= self.source.len() {
            return None;
        }

        Some(self.source.as_bytes()[self.current + 1])
    }

    fn consume(&mut self, token: Token) -> Option<()> {
        self.tokens.push(token);

        if self.current + 1 >= self.source.len() {
            return None;
        }

        self.current += 1;

        Some(())
    }

    fn parse_char(&self) -> Option<Token> {
        match self.source.as_bytes()[self.current] as char {
            '{' => Some(Token::LCurly),
            '}' => Some(Token::RCurly),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            ';' => Some(Token::Semicolon),
            ':' => Some(Token::Colon),
            '!' => Some(Token::Exclam),
            '?' => Some(Token::Question),
            '"' => Some(Token::Quotation),
            ' ' => Some(Token::Space),
            _ => None,
        }
    }

    fn parse_identifier(ch: u8) {}
}
