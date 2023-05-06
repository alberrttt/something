use super::super::prelude::*;
#[derive(Debug, ParseTokens, ParseTokensDisplay, Clone)]
pub struct VariableDeclaration {
    pub let_token: tokens::Let,
    pub name: Ident,
    pub equal: tokens::Equal,
    pub value: Literal,
    pub semicolon: tokens::Semicolon,
}
