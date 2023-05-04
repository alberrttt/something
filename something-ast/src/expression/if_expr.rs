

use something_dev_tools::ParseTokens;
use something_frontend_tokenizer::{tokens, Parse, Tokens};

use super::Expression;
use super::super::Statement;
#[derive(Debug, Clone)]
pub struct If {
    if_token: tokens::If,
    predicate: Box<Expression>,
    then_branch: Box<Statement>,
}
impl Parse for If {
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
        let if_token = input.parse()?;
        let predicate = input.parse()?;
        let then_branch = input.parse()?;
        Ok(Self {
            if_token,
            predicate,
            then_branch,
        })
    }
}
