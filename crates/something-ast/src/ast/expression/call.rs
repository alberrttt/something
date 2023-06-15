use crate::{tkn_recover, tokenizer::prelude::*};
use something_dev_tools::{ParseTokens, ParseTokensDisplay};
use Macros::Tkn;

use crate::ast::{delimiter::Parentheses, punctuated::Punctuated, tokenizer::TokenStream};

use super::Expression;

#[derive(Debug, Clone, ParseTokensDisplay)]
pub struct Call {
    pub ident: Ident,
    pub args: Parentheses<Punctuated<Expression, Tkn![,]>>,
}

impl Call {
    pub fn parse_with_ident(ident: Ident, input: &mut TokenStream) -> ParseResult<Self> {
        let delimiter = match input.peek() {
            Ok(ok) => ok,
            Err(_) => return Recoverable,
            Recoverable => return Recoverable,
        }
        .clone();
        let delimiter = match delimiter {
            Token::Parentheses(paren) => paren,
            _ => return Recoverable,
        };
        input.advance();

        Ok(Self {
            ident,
            args: Parentheses(
                delimiter.span,
                Punctuated::parse_terminated(&mut delimiter.tokens.clone().into())?,
            ),
        })
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
