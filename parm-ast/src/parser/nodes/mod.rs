use parm_common::Spanned;

use crate::prelude::{ErrorKind, Node, ParseResult};

use super::ParseStream;

pub mod declaration;
pub mod delimiter;
pub mod expression;
pub mod item;
pub mod path;
pub mod punctuated;
pub mod statement;
pub mod type_nodes;

#[derive(Debug, Clone, PartialEq)]
pub struct Empty;
impl<'a> Spanned for Empty {
    fn span(&self) -> parm_common::Span {
        parm_common::Span::default()
    }
}
impl<'a> Node<'a> for Empty {
    fn parse(parser: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        Ok(Self)
    }
}
