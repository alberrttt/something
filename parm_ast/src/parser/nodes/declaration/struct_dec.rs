use crate::prelude::*;
#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub struct Struct<'a> {
    pub struct_tkn: StructKeyword<'a>,
    pub ident: Identifier<'a>,
}

impl<'a> Node<'a> for Struct<'a> {
    fn parse(parser: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let struct_tkn = parser.step(|parser| StructKeyword::parse(parser).clone())?;
        let ident = parser.step(|parser| Identifier::parse(parser).clone())?;
        Ok(Self { struct_tkn, ident })
    }
}
