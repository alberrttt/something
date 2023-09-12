use std::slice::Iter;

use crate::tokenizer::{list::List, traits::AppendTokens, Parse, TokenStream};
use something_dev_tools::ParseTokensDisplay;

use crate::ast::{prelude::Brace, traits::Children, Node};

#[derive(Debug, Clone, ParseTokensDisplay)]
pub struct Block(pub Brace<List<Node>>);
impl AppendTokens for Block {
    fn append_tokens(&self, tokens: &mut TokenStream)
    where
        Self: Sized,
    {
        self.0.append_tokens(tokens);
    }
}
impl Block {
    pub fn iter(&self) -> Iter<Node> {
        self.0.inner.iter()
    }
}
use crate::prelude::*;
impl Parse for Block {
    fn parse(parser: &mut crate::parser::Parser) -> ParseResult<Self> {
        match Parse::parse(parser) {
            Ok(tmp) => Ok(Self(tmp)),
            Err(err) => Err(err),
        }
    }
}
impl Children<Node> for Block {
    fn children(&self) -> std::slice::Iter<Node> {
        self.0.iter()
    }
}
use something_dev_tools::item_name;
item_name!(Block, "block");
