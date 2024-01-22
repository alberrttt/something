use crate::ast::{parser::token_stream::TokenStream, prelude::*};
#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub struct Attribute<'a> {
    pub octothorpe: Octothorpe<'a>,
    pub bracket: Bracket<'a, TokenStream<'a>>,
}
impl<'a> Node<'a> for Attribute<'a> {
    fn parse(parser: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let octothorpe = parser.step(Octothorpe::parse)?;
        let bracket = parser.step(Bracket::parse)?;
        Ok(Self {
            octothorpe,
            bracket,
        })
    }
}
