use std::backtrace::Backtrace;
use std::fmt::Display;
use std::rc::Rc;

use colored::Colorize;

use crate::ast::prelude::*;
use crate::tokenizer::prelude::*;
#[derive(Debug, Clone)]
pub struct ParseError {
    pub surrounding: Option<TokenStream>,
    pub kind: ParseErrorKind,
    pub backtrace: Option<Rc<Backtrace>>,
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl ParseError {
    pub fn expected_token(expected: Token, got: Token) -> Self {
        todo!()
    }
    pub fn end_of_tokens() -> Self {
        todo!()
    }
    pub fn expected_token_stream(expected: TokenStream, got: TokenStream) -> Self {
        todo!()
    }
}
#[derive(Debug, Clone)]
pub enum ParseErrorKind {}
