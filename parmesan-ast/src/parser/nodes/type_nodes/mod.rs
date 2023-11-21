use std::alloc::Layout;

use crate::prelude::*;

use super::delimiter::Angle;
use parmesan_dev_macros::Spanned;

#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct TypeIdent<'a> {
    pub ident: Ident<'a>,
    pub generics: Option<Angle<'a, Punctuated<Ident<'a>, Comma<'a>>>>,
}
impl<'a> Node<'a> for TypeIdent<'a> {
    fn parse(
        parser: &mut crate::parser::ParseStream<'a>,
    ) -> Result<Self, crate::error::ParseError<'a>>
    where
        Self: Sized,
    {
        let ident = Ident::parse(parser)?;
        let generics = parser.step(|parser| {
            Angle::parse_manual(parser, |parser| Punctuated::parse_terminated(parser))
        });
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
    fn parse(
        parser: &mut crate::parser::ParseStream<'a>,
    ) -> Result<Self, crate::error::ParseError<'a>>
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
    fn parse(
        parser: &mut crate::parser::ParseStream<'a>,
    ) -> Result<Self, crate::error::ParseError<'a>>
    where
        Self: Sized,
    {
        let prefix = parser.step(|parser| ColonColon::parse(parser).clone());
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
    fn parse(
        parser: &mut crate::parser::ParseStream<'a>,
    ) -> Result<Self, crate::error::ParseError<'a>>
    where
        Self: Sized,
    {
        let borrow = parser.step(|parser| Borrow::parse(parser).clone());
        let path = TypePath::parse(parser)?;
        Ok(Self {
            borrow: borrow.ok(),
            path,
        })
    }
}
