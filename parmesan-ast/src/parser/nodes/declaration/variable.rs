use parmesan_common::Spanned;
use parmesan_dev_macros::Spanned;

use crate::{
    lexer::token::{Ident, Let},
    traits::Node,
};

#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct Variable<'a> {
    pub let_tkn: Let<'a>,
    pub ident: Ident<'a>,
}
impl<'a> Node<'a> for Variable<'a> {
    fn parse(parser: &mut crate::parser::Parser<'a>) -> Result<Self, crate::error::ParseError<'a>>
    where
        Self: Sized,
    {
        let let_tkn = parser.step(Let::parse)?;
        let ident = parser.step(Ident::parse)?;
        Ok(Self { let_tkn, ident })
    }
}
