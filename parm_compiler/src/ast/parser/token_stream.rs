use parm_common::Spanned;

use crate::ast::{
    prelude::{Node, ParseResult, Token},
    tree_display::TreeDisplay,
};

#[derive(Debug, Clone, PartialEq)]
pub struct TokenStream<'a>(&'a [Token<'a>]);
impl<'a> TreeDisplay for TokenStream<'a> {
    fn tree(&self) -> crate::ast::tree_display::Tree {
        crate::ast::tree_display::Tree::new("TokenStream")
    }
}
impl<'a> Spanned for TokenStream<'a> {
    fn span(&self) -> parm_common::Span {
        self.0
            .first()
            .unwrap()
            .span()
            .join(self.0.last().unwrap().span())
    }
}

impl<'a> Node<'a> for TokenStream<'a> {
    fn parse(parser: &mut crate::ast::prelude::ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        parser.current = parser.tokens.len();
        Ok(Self(parser.tokens))
    }
}
