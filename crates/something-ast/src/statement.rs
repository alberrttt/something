use something_dev_tools::{item_name, ParseTokens, ParseTokensDisplay};
use something_frontend_tokenizer::{
    tokens::{Return, Semicolon, Token},
    Parse, Token, Tokens,
};

use crate::{declaration::Declaration, expression::Expression};

#[derive(ParseTokensDisplay, Debug, Clone, ParseTokens)]
pub enum Statement {
    Expression(Expression, Semicolon),
    Return(Return, Expression, Semicolon),
}
item_name!(Statement, "statement");
