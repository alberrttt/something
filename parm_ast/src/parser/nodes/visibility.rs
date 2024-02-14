use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub struct Visibility<'a> {
    pub visibility: Option<Pub<'a>>,
}

impl<'a> Node<'a> for Visibility<'a> {
    fn parse(parser: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let visibility = parser.step(Pub::parse).ok();
        Ok(Self { visibility })
    }
}
