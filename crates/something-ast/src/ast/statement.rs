use crate::tokenizer::*;
use something_dev_tools::{item_name, ParseTokens, ParseTokensDisplay};

use crate::ast::expression::Expression;
use crate::tokenizer::prelude::*;
#[derive(ParseTokensDisplay, Debug, Clone)]
pub enum Statement {
    Expression((Expression, Semicolon)),
    Return((Return, Expression, Semicolon)),
}
impl Parse for Statement {
    fn parse(input: &mut Tokens) -> ParseResult<Self> {
        match input.step(|input| Parse::parse(input)) {
            Ok(variant) => return Ok(Statement::Expression(variant)),
            Err(err) => {
                return Err(err);
            }
            Recoverable => {}
        }
        match input.step(|input| Parse::parse(input)) {
            Ok(variant) => return Ok(Statement::Return(variant)),
            Err(err) => {
                return Err(err);
            }
            Recoverable => {}
        }
        Recoverable
    }
}
impl Parse for Box<Statement> {
    fn parse(input: &mut Tokens) -> ParseResult<Self> {
        Ok(Box::new(Statement::parse(input)?))
    }
}
item_name!(Statement, "statement");
