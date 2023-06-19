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
    pub fn parse_with_ident(ident: Ident, parser: &mut crate::parser::Parser) -> ParseResult<Self> {
        let args: Paren<Punctuated<Expression, Comma>> = Paren::parse(parser)?;
        Ok(Self { ident, args })
    }
}

impl Parse for Call {
    fn parse(parser: &mut crate::parser::Parser) -> ParseResult<Self> {
        let ident = Ident::parse(parser)?;
        Self::parse_with_ident(ident, parser)
    }
}

use something_dev_tools::item_name;
item_name!(Call, "call");
