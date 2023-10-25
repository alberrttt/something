use crate::lexer::token::Token;
pub mod nodes;
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Parser<'a> {
    src: &'a str,
    tokens: &'a [Token<'a>],
    /// The index of the current token
    current: usize,
}

impl<'a> Parser<'a> {}
