use parmesan_common::Spanned;

use crate::prelude::{Node, ParseError};

use super::ParseStream;

pub mod declaration;
pub mod delimiter;
pub mod expression;
pub mod punctuated;
pub mod statement;

#[derive(Debug, Clone, PartialEq)]
pub struct Empty;
impl<'a> Spanned for Empty {
    fn span(&self) -> parmesan_common::Span {
        parmesan_common::Span::default()
    }
}
impl<'a> Node<'a> for Empty {
    fn parse(parser: &mut ParseStream<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        Ok(Self)
    }
}
