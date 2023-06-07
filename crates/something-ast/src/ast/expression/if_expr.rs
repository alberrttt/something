use crate::tokenizer::prelude::*;

use super::super::prelude::*;
#[derive(Debug, Clone, ParseTokensDisplay)]
pub struct If {
    if_token: token::If,
    predicate: Box<Expression>,
    then_branch: Box<Expression>,
}
pub enum ThenBlock {
    Statement(Statement),
    Block(Braces<List<Node>>),
}
impl Parse for If {
    fn parse(input: &mut Tokens) -> Result<Self, ParseError> {
        let if_token = input.parse()?;
        let predicate = input.parse().unwrap();
        let then_branch = input.parse().unwrap();

        Ok(Self {
            if_token,
            predicate,
            then_branch,
        })
    }
}
use something_dev_tools::item_name;
item_name!(If, "if");
