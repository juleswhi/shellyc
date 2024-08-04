#[allow(dead_code)]
pub enum Token {
    Import(ImportType)
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
            current: 0
        }
    }
}

impl Lexer {
    pub fn lex(&mut self) {
        loop {

        }
    }
    fn peek(&self) -> Option<u8> {
        if self.current + 1 >= self.source.len() {
            return None;
        }

        Some(self.source.as_bytes()[self.current+1])
    }

    fn consume(&mut self, token: Token) -> Option<()> {
        self.tokens.push(token);

        if self.current + 1 >= self.source.len() {
            return None;
        }

        self.current += 1;

        Some(())
    }
}
