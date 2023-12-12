use crate::prelude::*;

use super::delimiter::Angle;
use parm_dev_macros::Spanned;

#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct TypeIdent<'a> {
    pub ident: Identifier<'a>,
    pub generics: Option<Angle<'a, Punctuated<Identifier<'a>, Comma<'a>>>>,
}
impl<'a> Node<'a> for TypeIdent<'a> {
    fn parse(parser: &mut crate::parser::ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let ident = Identifier::parse(parser)?;
        let generics =
            parser.step(|parser| Angle::parse_manual(parser, Punctuated::parse_terminated));
        Ok(Self {
            ident,
            generics: generics.ok(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct Borrow<'a> {
    pub ampersand: Amper<'a>,
    pub mutable: Option<Mut<'a>>,
}
impl<'a> Node<'a> for Borrow<'a> {
    fn parse(parser: &mut crate::parser::ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let ampersand = parser.step(|parser| Amper::parse(parser).clone())?;
        let mutable = parser.step(|parser| Mut::parse(parser).clone());
        Ok(Self {
            ampersand,
            mutable: mutable.ok(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct TypePath<'a> {
    pub prefix: Option<ColonColon<'a>>,
    pub segments: Punctuated<TypeIdent<'a>, ColonColon<'a>>,
}
impl<'a> Node<'a> for TypePath<'a> {
    fn parse(parser: &mut crate::parser::ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let prefix = parser.step(ColonColon::parse);
        let segments: Punctuated<TypeIdent<'_>, ColonColon<'_>> =
            Punctuated::parse_terminated_expect(parser)?;
        Ok(Self {
            prefix: prefix.ok(),
            segments,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct TypeExpression<'a> {
    pub borrow: Option<Borrow<'a>>,
    pub path: TypePath<'a>,
}
impl<'a> Node<'a> for TypeExpression<'a> {
    fn parse(parser: &mut crate::parser::ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let borrow = parser.step(Borrow::parse);
        let path = TypePath::parse(parser)?;
        Ok(Self {
            borrow: borrow.ok(),
            path,
        })
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct TypeAnnotation<'a> {
    pub colon: Colon<'a>,
    pub ty: TypeExpression<'a>,
}
impl<'a> Node<'a> for TypeAnnotation<'a> {
    fn parse(parser: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let colon = parser.step(|parser| Colon::parse(parser).clone())?;
        let ty = parser.step(|parser| TypeExpression::parse(parser).clone())?;
        Ok(Self { colon, ty })
    }
}
