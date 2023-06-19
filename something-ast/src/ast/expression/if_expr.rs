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
    fn parse(parser: &mut crate::parser::Parser) -> ParseResult<Self> {
        let if_token = parser.parse()?;
        let predicate = parser.parse().unwrap();
        let then_branch = parser.parse().unwrap();
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
