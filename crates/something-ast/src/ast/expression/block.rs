use std::{backtrace::Backtrace, slice::Iter};

use crate::tokenizer::{list::List, prelude::ParseError, traits::AppendTokens, Parse, Tokens};
use something_dev_tools::{ParseTokens, ParseTokensDisplay};

use crate::ast::{
    prelude::{Braces, Statement},
    traits::Children,
    Node,
};

#[derive(Debug, Clone, ParseTokensDisplay)]
pub struct Block(pub Braces<List<Node>>);
impl AppendTokens for Block {
    fn append_tokens(&self, tokens: &mut Tokens)
    where
        Self: Sized,
    {
        self.0.append_tokens(tokens);
    }
}
impl Block {
    pub fn iter(&self) -> Iter<Node> {
        self.0 .1.iter()
    }
}
impl Parse for Block {
    fn parse(input: &mut Tokens) -> Result<Self, ParseError> {
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
