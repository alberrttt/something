use something_dev_tools::{ParseTokens, ParseTokensDisplay};
use something_frontend_tokenizer::{ident::Ident, prelude::ParseError, tokens, Parse};

use crate::{delimiter::Parentheses, punctuated::Punctuated};

use super::Expression;

#[derive(Debug, Clone, ParseTokensDisplay)]
pub struct Call {
    pub ident: Ident,
    pub args: Parentheses<Punctuated<Expression, tokens::Comma>>,
}
impl Parse for Call {
    fn parse(input: &mut something_frontend_tokenizer::Tokens) -> Result<Self, ParseError> {
        let ident = Ident::parse(input)?;
        let delimiter = match input.advance() {
            Some(tokens::Token::Parentheses(paren)) => paren,
            _ => {
                return Err(ParseError::Generic(
                    format!("Expected Parentheses, got {:?}", input.advance().clone()).into(),
                ))
            }
        };

        Ok(Self {
            ident,
            args: Parentheses(
                delimiter.span,
                Punctuated::parse_terminated(&mut delimiter.tokens.clone().into())?,
            ),
        })
    }
}
use something_dev_tools::item_name;
item_name!(Call, "call");
