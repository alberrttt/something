use crate::lexer::token::{self, Let, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Function<'a> {
    pub fn_tkn: token::FnKeyword<'a>,
}
