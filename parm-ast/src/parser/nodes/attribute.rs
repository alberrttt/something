use crate::{parser::token_stream::TokenStream, prelude::*};
#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct Attribute<'a> {
    pub octothorpe: Octothorpe<'a>,
    pub bracket: Bracket<'a, TokenStream<'a>>,
}
impl<'a> Node<'a> for Attribute<'a> {
    fn parse(parser: &mut ParseStream<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        let octothorpe = parser.step(|parser| Octothorpe::parse(parser).clone())?;
        let bracket = parser.step(|parser| Bracket::parse(parser).clone())?;
        Ok(Self {
            octothorpe,
            bracket,
        })
    }
}
