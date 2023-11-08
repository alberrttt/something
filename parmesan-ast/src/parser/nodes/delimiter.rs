use parmesan_dev_macros::Spanned;

use crate::{
    lexer::token::{LBrace, RBrace},
    traits::{self, Node},
};
use parmesan_common::Spanned;
#[derive(Debug, Clone, PartialEq)]
pub struct Brace<'a, T>
where
    T: Node<'a>,
{
    pub open: LBrace<'a>,
    pub inner: T,
    pub close: RBrace<'a>,
}
impl<'a, T: Spanned + traits::Node<'a>> Spanned for Brace<'a, T> {
    fn span(&self) -> parmesan_common::Span {
        (self.open.span(), self.close.span()).into()
    }
}
impl<'a, T: Spanned + Node<'a>> Node<'a> for Brace<'a, T> {
    fn parse<'b: 'a>(
        parser: &'b mut crate::parser::Parser<'a>,
    ) -> Result<Self, crate::error::ParseError<'a>>
    where
        Self: Sized,
    {
        let open = parser.step(LBrace::parse)?;
        let inner = parser.step(T::parse).unwrap();
        let close = parser.step(RBrace::parse).unwrap();
        Ok(Self { open, inner, close })
    }
}

#[test]
fn test_delimiter() {}
