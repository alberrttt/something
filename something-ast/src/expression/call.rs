use something_dev_tools::ParseTokens;
use something_frontend_tokenizer::ident::Ident;

use crate::delimiter::Parenthesis;

use super::Expression;

#[derive(Debug, ParseTokens, Clone)]
pub struct Call {
    pub ident: Ident,
    pub args: Parenthesis<Expression>,
}
