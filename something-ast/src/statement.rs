use something_dev_tools::{ParseTokens, ParseTokensDisplay};
use something_frontend_tokenizer::tokens::Semicolon;

use crate::{declaration::Declaration, expression::Expression};

#[derive(ParseTokensDisplay, Debug, Clone)]
pub enum Statement {
    Declaration(Declaration),
    Expression(Expression, Semicolon),
}
use something_frontend_tokenizer::Parse;
use something_frontend_tokenizer::Tokens;
use std::fmt::{Display, Formatter};
impl Parse for Statement {
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
        let mut err: Box<dyn std::error::Error> = "".into();
        match input.step(|input| Parse::parse(input)) {
            Ok(variant) => return Ok(Statement::Declaration(variant)),
            Err(x) => {
                err = x;
            }
        }
        match input.step(|input| Parse::parse(input)) {
            Ok(variant) => {
                dbg!(&variant);
                return Ok(Statement::Expression(
                    variant,
                    input.step(|input| Parse::parse(input)).unwrap(),
                ));
            }
            Err(x) => {
                err = x;
            }
        }
        Err(err)
    }
}
impl Parse for Box<Statement> {
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Box::new(Statement::parse(input)?))
    }
}
