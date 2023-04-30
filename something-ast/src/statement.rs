use something_dev_tools::ParseTokens;
use something_frontend_tokenizer::tokens::Semicolon;

use crate::{declaration::Declaration, expression::Expression};

#[derive(Debug)]
pub enum Statement {
    Declaration(Declaration),
    Expression(Expression, Semicolon),
}
mod __Statement {
    use super::Statement;
    use something_frontend_tokenizer::tokens::Parse;
    use something_frontend_tokenizer::Tokens;
    impl Parse for Statement {
        fn parse(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
            let mut err: Box<dyn std::error::Error> = "".into();
            match input.step(|input| Parse::parse(input)) {
                Ok(variant) => return Ok(Statement::Declaration(variant)),
                Err(x) => err = x,
            }
            match input.step(|input| Parse::parse(input)) {
                Ok(variant) => {
                    return Ok(Statement::Expression(
                        variant,
                        input.step(|input| Parse::parse(input))?,
                    ))
                }
                Err(x) => err = x,
            }
            Err(err)
        }
    }
}
pub use __Statement::*;
