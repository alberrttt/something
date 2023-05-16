use something_dev_tools::{item_name, ParseTokens, ParseTokensDisplay};
use something_frontend_tokenizer::{tokens::Semicolon, Parse};

use crate::{declaration::Declaration, expression::Expression};

#[derive(ParseTokensDisplay, Debug, Clone)]
pub struct Statement(Expression, Semicolon);
item_name!(Statement, "statement");
impl Parse for Statement {
    fn parse(
        input: &mut something_frontend_tokenizer::Tokens,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let expr = input.parse()?;
        let semicolon = input.parse().unwrap();
        Ok(Self(expr, semicolon))
    }
}
