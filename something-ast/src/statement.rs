use something_dev_tools::ParseTokens;
use something_frontend_tokenizer::tokens::Semicolon;

use crate::{declaration::Declaration, expression::Expression};

#[derive(Debug, ParseTokens, Clone)]
pub enum Statement {
    Declaration(Declaration),
    Expression(Expression, Semicolon),
}
