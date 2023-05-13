use something_dev_tools::{ParseTokens, ParseTokensDisplay};
use something_frontend_tokenizer::{list::List, Parse, Tokens};

use crate::{
    prelude::{Braces, Statement},
    Node,
};

#[derive(Debug, Clone, ParseTokensDisplay)]
pub struct Block(pub Braces<List<Node>>);
impl Parse for Block {
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
        let tmp = input.step(|input| Parse::parse(input));
        dbg!(&tmp);
        match tmp {
            Ok(tmp) => Ok(Self(tmp)),
            Err(err) => Err(err),
        }
    }
}
