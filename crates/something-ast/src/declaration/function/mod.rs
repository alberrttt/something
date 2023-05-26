use std::fmt::Display;

use something_dev_tools::ParseTokensDisplay;
use something_frontend_tokenizer::{list::List, Parse};

use crate::{attribute::Attribute, expression::block::Block, punctuated::Punctuated};

use self::return_type::ReturnType;
use super::super::prelude::*;

#[derive(Debug, Clone, ParseTokens, ParseTokensDisplay)]
pub struct FunctionDeclaration {
    // pub modifiers: Option<Attribute>,
    pub fn_token: tokens::Fn,
    pub name: Ident,
    pub params: Parentheses<Punctuated<(Ident, Ident), Comma>>,
    pub body: Block,
    pub return_type: ReturnType,
}

impl Display for FunctionDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // if let Some(modifiers) = &self.modifiers {
        //     write!(f, "{} ", modifiers)?;
        // }
        write!(
            f,
            "{} {}{}",
            self.fn_token,
            self.name,
            self.params
                .iter()
                .enumerate()
                .map(|(i, (name, _))| { format!("{}: {},", name.0, name.1) })
                .collect::<String>()
        )?;
        write!(f, "{}", self.return_type)?;
        write!(
            f,
            "{}",
            self.body
                .iter()
                .map(|f| { f.display() })
                .collect::<String>()
        )
    }
}
use something_dev_tools::item_name;
item_name!(FunctionDeclaration, "function declaration");
pub mod return_type;
