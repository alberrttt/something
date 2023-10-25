use parmesan_common::Spanned;

use crate::traits::Parse;

use self::{ident::Identifier, number::Number};

use super::Node;

pub mod binary;
pub mod ident;
pub mod number;
#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Identifier(ident::Identifier<'a>),
    Number(number::Number<'a>),
    BinaryExpression(binary::BinaryExpression<'a>),
}

impl Spanned for Expression<'_> {
    fn span(&self) -> parmesan_common::Span {
        use Expression::*;
        match self {
            Identifier(ident) => ident.span(),
            Number(number) => number.span(),
            BinaryExpression(binary) => binary.span(),
        }
    }
}
impl Parse for Expression<'_> {
    fn parse<'src>(
        parser: &mut crate::parser::Parser<'src>,
    ) -> Result<Self, Box<dyn crate::error::ParseError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
