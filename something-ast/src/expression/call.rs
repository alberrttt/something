use something_dev_tools::ParseTokens;
use something_frontend_tokenizer::{ident::Ident, tokens, Parse};

use crate::{delimiter::Parenthesis, punctuated::Punctuated};

use super::Expression;

#[derive(Debug, Clone)]
pub struct Call {
    pub ident: Ident,
    pub args: Parenthesis<Punctuated<Expression, tokens::Comma>>,
}
impl Parse for Call {
    fn parse(
        input: &mut something_frontend_tokenizer::Tokens,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let ident = Ident::parse(input)?;
        let delimiter = match input.advance() {
            Some(tokens::Token::Paren(paren)) => paren,
            _ => {
                return Err(
                    format!("Expected Parenthesis, got {:?}", input.advance().clone()).into(),
                )
            }
        };

        Ok(Self {
            ident,
            args: Parenthesis(
                delimiter.span,
                Punctuated::parse_terminated(&mut delimiter.tokens.clone().into())?,
            ),
        })
    }
}
