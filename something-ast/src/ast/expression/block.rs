use crate::ast::statement::Statement;
pub use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub open_brace: token::LeftBrace,
    pub statements: Vec<Statement>,
    pub close_brace: token::RightBrace,
}
