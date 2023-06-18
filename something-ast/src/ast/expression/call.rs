use crate::tokenizer::prelude::*;
use something_dev_tools::ParseTokensDisplay;
use Macros::Tkn;

use crate::ast::{delimiter::Paren, punctuated::Punctuated, tokenizer::TokenStream};

use super::Expression;

#[derive(Debug, Clone, ParseTokensDisplay)]
pub struct Call {
    pub ident: Ident,
    pub args: Paren<Punctuated<Expression, Tkn![,]>>,
}

impl Call {
    pub fn parse_with_ident(ident: Ident, input: &mut TokenStream) -> ParseResult<Self> {
        let args: Paren<Punctuated<Expression, Comma>> = Paren::parse(input)?;
        Ok(Self { ident, args })
    }
}

impl Parse for Call {
    fn parse(input: &mut TokenStream) -> ParseResult<Self> {
        let ident = Ident::parse(input)?;
        Self::parse_with_ident(ident, input)
    }
}

use something_dev_tools::item_name;
item_name!(Call, "call");
