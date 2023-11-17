use std::alloc::Layout;

use crate::prelude::{Comma, Ident, Node, Punctuated, Spanned};

use super::nodes::delimiter::Angle;
use parmesan_dev_macros::Spanned;

#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct TypeIdent<'a> {
    pub ident: Ident<'a>,
}
impl<'a> Node<'a> for TypeIdent<'a> {
    fn parse(
        parser: &mut crate::prelude::ParseStream<'a>,
    ) -> Result<Self, crate::prelude::ParseError<'a>>
    where
        Self: Sized,
    {
        let ident = Ident::parse(parser)?;
        Ok(Self { ident })
    }
}
#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct Type<'a> {
    pub ident: TypeIdent<'a>,
    pub generics: Option<Angle<'a, Punctuated<TypeIdent<'a>, Comma<'a>>>>,
}

