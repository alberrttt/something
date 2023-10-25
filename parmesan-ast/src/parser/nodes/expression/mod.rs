use parmesan_common::Spanned;

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
