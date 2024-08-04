use token::Lexer;

mod token;
pub fn main() {
    _ = Lexer::from("import std".into()).lex();
}
