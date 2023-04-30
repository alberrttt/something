use something_dev_tools::ParseTokens;
use something_frontend_tokenizer::{
    ident::Ident,
    lit::Literal,
    tokens::{self, Parse},
    Token,
};
#[derive(Debug, ParseTokens)]
pub struct VariableDeclaration {
    pub let_token: tokens::Let,
    pub name: Ident,
    pub equal: tokens::Equal,
    pub value: Literal,
    pub semicolon: tokens::Semicolon,
}
