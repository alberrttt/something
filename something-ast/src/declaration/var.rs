use super::super::prelude::*;
#[derive(Debug, ParseTokensDisplay, Clone, ParseTokens)]
pub struct VariableDeclaration {
    pub let_token: tokens::Let,
    pub name: Ident,
    pub equal: tokens::Equal,
    pub value: Expression,
    pub semicolon: tokens::Semicolon,
}
use something_dev_tools::item_name;
item_name!(VariableDeclaration, "variable declaration");
