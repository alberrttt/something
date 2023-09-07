mod use_stmt;
use crate::ast::path::var;
use crate::tokenizer::prelude::*;
use crate::{ast::expression::Expression, parser};
use something_dev_tools::{item_name, ParseTokens, ParseTokensDisplay};
use Macros::Tkn;
#[derive(ParseTokensDisplay, Debug, Clone)]
pub enum Statement {
    Expression((Expression, Tkn![;])),
    Return((Tkn![Return], Expression, Tkn![;])),
}
// This might be a spot that i mess UP
impl Parse for Statement {
    fn parse(parser: &mut crate::parser::Parser) -> ParseResult<Self> {
        match parser.step::<Expression>(|parser| Parse::parse(parser)) {
            Ok(variant) => {
                let semicolon = if matches!(variant, Expression::If(_) | Expression::Block(_)) {
                    Semicolon {
                        span: Span::default(),
                    }
                } else {
                    parser.step::<Semicolon>(|parser| Parse::parse(parser))?
                };
                return Ok(Statement::Expression((variant, semicolon)));
            }
            Err(_) | Recoverable => {}
        }
        match parser.step(|parser| Parse::parse(parser)) {
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
    fn parse(parser: &mut crate::parser::Parser) -> ParseResult<Self> {
        Ok(Box::new(Statement::parse(parser)?))
    }
}
item_name!(Statement, "statement");
