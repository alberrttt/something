use something_dev_tools::ParseTokens;
use something_frontend_tokenizer::{
    ident::Ident,
    tokens::{self, Colon, Parse, Token},
    Token, Tokens,
};

use crate::{
    delimiter::{Braces, Brackets, Parenthesis},
    expression::Expression,
    punctuated::Punctuated,
    Statement,
};

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub modifiers: Brackets<Ident>,
    pub colon: Colon,
    pub fn_token: tokens::Fn,
    pub name: Ident,
    pub params: Parenthesis<Ident>, // todo
    pub body: Braces<Punctuated<Expression, tokens::Semicolon>>, // todo
}
impl Parse for FunctionDeclaration {
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
        dbg!(input.peek());
        let modifiers = Brackets::parse(input)?;

        let colon = Colon::parse(input)?;
        let fn_token = tokens::Fn::parse(input)?;
        let name = Ident::parse(input)?;
        let params = Parenthesis::parse(input)?;
        let body = Braces::parse(input)?;
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
