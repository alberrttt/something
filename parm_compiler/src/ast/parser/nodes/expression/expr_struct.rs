use crate::ast::{parser::nodes::declaration::struct_dec::StructMemberDeclaration, prelude::*};
#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub struct StructExpression<'a> {
    pub ident: Identifier<'a>,
    pub body: Brace<'a, Punctuated<StructMemberInitialization<'a>, Comma<'a>>>,
}

#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub struct StructMemberInitialization<'a> {
    pub ident: Identifier<'a>,
    pub colon: Colon<'a>,
    pub expr: Expression<'a>,
}
impl<'a> Node<'a> for StructMemberInitialization<'a> {
    fn parse(parser: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let ident = parser.step(|parser| Identifier::parse(parser).clone())?;
        parser.panic = true;
        let colon = parser.step(|parser| Colon::parse(parser).clone())?;
        let expr = parser.step(|parser| Expression::parse(parser).clone())?;
        parser.panic = false;
        Ok(Self { ident, colon, expr })
    }
}
