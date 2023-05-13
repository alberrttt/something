use something_frontend_tokenizer::{list::List, Parse};

use super::super::prelude::*;
#[derive(Debug, Clone, ParseTokensDisplay)]
pub struct If {
    if_token: tokens::If,
    predicate: Box<Expression>,
    then_branch: Box<Expression>,
}
pub enum ThenBlock {
    Statement(Statement),
    Block(Braces<List<Node>>),
}
impl Parse for If {
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
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
