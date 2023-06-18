mod use_stmt;
use crate::ast::expression::Expression;
use crate::tokenizer::prelude::*;
use something_dev_tools::{item_name, ParseTokens, ParseTokensDisplay};
use Macros::Tkn;
#[derive(ParseTokensDisplay, Debug, Clone, ParseTokens)]
pub enum Statement {
    Expression((Expression, Tkn![;])),
    Return((Tkn![Return], Expression, Tkn![;])),
}

item_name!(Statement, "statement");
