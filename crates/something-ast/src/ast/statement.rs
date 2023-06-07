use crate::tokenizer::*;
use something_dev_tools::{item_name, ParseTokens, ParseTokensDisplay};

use crate::ast::expression::Expression;
use crate::tokenizer::prelude::*;
#[derive(ParseTokensDisplay, Debug, Clone, ParseTokens)]
pub enum Statement {
    Expression(Expression, Semicolon),
    Return(Return, Expression, Semicolon),
}
item_name!(Statement, "statement");
