use std::backtrace::Backtrace;
use std::fmt::Display;
use std::rc::Rc;

use colored::Colorize;
use log::Log;

use crate::ast::prelude::*;
use crate::tokenizer::prelude::*;
#[derive(Debug, Clone, Default)]
pub struct ParseError {
    pub surrounding: Option<TokenStream>,
    pub kind: ParseErrorKind,
    pub backtrace: Option<Rc<Backtrace>>,
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ParseErrorKind::ExpectedToken(expected) => {
                let log = Log::default();
                write!(f, "{}", log)
            }
            ParseErrorKind::Todo => todo!(),
        }
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
#[derive(Debug, Clone, Default)]
pub enum ParseErrorKind {
    ExpectedToken(Token),
    #[default]
    Todo,
}
