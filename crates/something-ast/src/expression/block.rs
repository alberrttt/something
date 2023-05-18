use std::{backtrace::Backtrace, slice::Iter};

use something_dev_tools::{ParseTokens, ParseTokensDisplay};
use something_frontend_tokenizer::{list::List, Parse, Tokens};

use crate::{
    prelude::{Braces, Statement},
    traits::Children,
    Node,
};

#[derive(Debug, Clone, ParseTokensDisplay)]
pub struct Block(pub Braces<List<Node>>);
impl Block {
    pub fn iter(&self) -> Iter<Node> {
        self.0 .1.iter()
    }
}
impl Parse for Block {
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
        let tmp = input.step(|input| Parse::parse(input));
        match tmp {
            Ok(tmp) => Ok(Self(tmp)),
            Err(err) => Err(err),
        }
    }
}
impl Children<Node> for Block {
    fn children(&self) -> std::slice::Iter<Node> {
        self.0 .1.iter()
    }
}
use something_dev_tools::item_name;
item_name!(Block, "block");
