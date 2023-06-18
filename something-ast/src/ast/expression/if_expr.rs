use crate::tokenizer::prelude::*;

use super::super::prelude::*;
#[derive(Debug, Clone, ParseTokensDisplay)]
pub struct If {
    if_token: Tkn![If],
    predicate: Box<Expression>,
    then_branch: Box<Expression>,
}
pub enum ThenBlock {
    Statement(Statement),
    Block(Brace<List<Node>>),
}
impl Parse for If {
    fn parse(input: &mut TokenStream) -> ParseResult<Self> {
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
use Macros::Tkn;
item_name!(If, "if");
