use something_dev_tools::ParseTokens;
use something_frontend_tokenizer::{
    ident::Ident,
    tokens::{self, Colon, Parse, Token},
    Token, Tokens,
};

use crate::{
    delimiter::{Braces, Brackets, Parenthesis},
    Statement,
};

#[derive(Debug, ParseTokens)]
pub struct FunctionDeclaration {
    pub modifiers: Brackets<Ident>,
    pub colon: Colon,
    pub fn_token: tokens::Fn,
    pub name: Ident,
    pub params: Parenthesis<Ident>, // todo
    pub body: Braces<Statement>,    // todo
}
