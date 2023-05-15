use something_dev_tools::ParseTokensDisplay;
use something_frontend_tokenizer::{list::List, Parse};

use crate::{attribute::Attribute, expression::block::Block};

use self::return_type::ReturnType;
use super::super::prelude::*;

#[derive(Debug, Clone, ParseTokens, ParseTokensDisplay)]
pub struct FunctionDeclaration {
    // pub modifiers: Option<Attribute>,
    pub fn_token: tokens::Fn,
    pub name: Ident,
    pub params: Parentheses<List<(Ident, tokens::Colon, Ident)>>,
    pub body: Block,
    pub return_type: ReturnType,
}
use something_dev_tools::item_name;
item_name!(FunctionDeclaration, "function declaration");
pub mod return_type;
