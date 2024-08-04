use std::fs;
use token::Lexer;

mod token;

const FILE_PATH: &str = "examples/main.shl";
pub fn main() {
    let contents = fs::read_to_string(FILE_PATH).unwrap();
    _ = Lexer::from(contents).lex();
}
