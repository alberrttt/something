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

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub modifiers: Brackets<Ident>,
    pub colon: Colon,
    pub fn_token: tokens::Fn,
    pub name: Ident,
    pub params: Parenthesis<Ident>, // todo
    pub body: Braces<Statement>,    // todo
}
impl Parse for FunctionDeclaration {
    fn parse(
        input: &mut something_frontend_tokenizer::Tokens,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let modifiers = Parse::parse(input)?;
        let colon = Parse::parse(input)?;
        let fn_token = Parse::parse(input)?;
        let name = Parse::parse(input)?;
        let params = Parse::parse(input)?;
        let body = Parse::parse(input)?;
        Ok(Self {
            modifiers,
            colon,
            fn_token,
            name,
            params,
            body,
        })
    }
}
