use something_dev_tools::{ParseTokens, ParseTokensDisplay};
use something_frontend_tokenizer::tokens::Semicolon;

use crate::{declaration::Declaration, expression::Expression};

#[derive(ParseTokensDisplay, Debug, Clone, ParseTokens)]
pub struct Statement(Expression, Semicolon);
