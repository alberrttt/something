use crate::{lexer::Lexer, parser::Parser};

#[test]
fn test_file() {
    let src = include_str!("main.txt");
    let mut lexer = Lexer::from(src);
    let tokens = lexer.lex();

    let mut parser = Parser {
        src,
        tokens,
        current: 0,
    };
}
