use crate::lexer::token::Let;

#[derive(Debug, Clone, PartialEq)]
pub struct Variable<'a> {
    pub let_tkn: Let<'a>,
}
