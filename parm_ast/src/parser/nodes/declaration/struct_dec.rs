use crate::{parser::nodes::visibility::Visibility, prelude::*};
#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub struct StructDeclaration<'a> {
    pub visibility: Visibility<'a>,
    pub struct_tkn: StructKeyword<'a>,
    pub ident: Identifier<'a>,
    pub body: Brace<'a, Punctuated<StructMemberDeclaration<'a>, Comma<'a>>>,
}

impl<'a> Node<'a> for StructDeclaration<'a> {
    fn parse(parser: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let visibility = parser.step(Visibility::parse)?;
        let struct_tkn = parser.step(StructKeyword::parse)?;
        parser.panic = true;
        let ident = parser.step(Identifier::parse)?;
        let body = parser.step(Brace::parse)?;
        parser.panic = false;
        Ok(Self {
            visibility,
            struct_tkn,
            ident,
            body,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub struct StructMemberDeclaration<'a> {
    pub ident: Identifier<'a>,
    pub colon: Colon<'a>,
    pub ty: TypeExpression<'a>,
}
impl<'a> Node<'a> for StructMemberDeclaration<'a> {
    fn parse(parser: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let ident = parser.step(|parser| Identifier::parse(parser).clone())?;
        let colon = parser.step(|parser| Colon::parse(parser).clone())?;
        let ty = parser.step(|parser| TypeExpression::parse(parser).clone())?;

        Ok(Self { ident, colon, ty })
    }
}
