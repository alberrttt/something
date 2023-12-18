use std::fmt::format;

use parm_dev_macros::Spanned;

use crate::{parser::ast_displayer::DisplayNode, prelude::*, traits::CreateDisplayNode};

use super::{expr, precedence::Precedence, Expression};

#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct Group<'a> {
    pub paren: Paren<'a, Box<Expression<'a>>>,
}
impl<'a> CreateDisplayNode for Group<'a> {
    fn create_display_node(&self) -> DisplayNode {
        DisplayNode::new("Group").child(self.paren.inner.create_display_node())
    }
}
impl<'a> Node<'a> for Group<'a> {
    fn parse(parser: &mut ParseStream<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        let paren: Paren<'a, Box<Expression<'a>>> = Paren::parse_manual(parser, |parser| {
            let expr = expr(parser, Precedence::Assignment)?;
            Ok(Box::new(expr))
        })?;
        Ok(Self { paren })
    }
}
