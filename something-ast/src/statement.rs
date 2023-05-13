use something_dev_tools::{ParseTokens, ParseTokensDisplay};
use something_frontend_tokenizer::{tokens::Semicolon, Parse};

use crate::{declaration::Declaration, expression::Expression};

#[derive(ParseTokensDisplay, Debug, Clone)]
pub struct Statement(Expression, Semicolon);
impl Parse for Statement {
    fn parse(
        input: &mut something_frontend_tokenizer::Tokens,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let expr = input.parse()?;
        dbg!(&expr);
        let semicolon = input.parse()?;
        Ok(Self(expr, semicolon))
    }
}
