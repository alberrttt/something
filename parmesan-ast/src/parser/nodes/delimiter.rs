use parmesan_dev_macros::Spanned;

use crate::{
    lexer::token::*,
    parser::Parser,
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
    fn parse(
        parser: &mut crate::parser::ParseStream<'a>,
    ) -> Result<Self, crate::error::ParseError<'a>>
    where
        Self: Sized,
    {
        let open = parser.step(LBrace::parse)?;
        let inner = parser.step(T::parse)?;
        let close = parser.step(RBrace::parse)?;
        Ok(Self { open, inner, close })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Paren<'a, T: Node<'a> + Spanned> {
    pub open: LParen<'a>,
    pub inner: T,
    pub close: RParen<'a>,
}

impl<'a, T: Spanned + Node<'a>> Spanned for Paren<'a, T> {
    fn span(&self) -> parmesan_common::Span {
        (self.open.span(), self.close.span()).into()
    }
}

impl<'a, T: Spanned + Node<'a>> Node<'a> for Paren<'a, T> {
    fn parse(
        parser: &mut crate::parser::ParseStream<'a>,
    ) -> Result<Self, crate::error::ParseError<'a>>
    where
        Self: Sized,
    {
        let open = parser.step(LParen::parse)?;
        let inner = parser.step(T::parse)?;
        let close = parser.step(RParen::parse)?;
        Ok(Self { open, inner, close })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bracket<'a, T: Node<'a> + Spanned> {
    pub open: LBracket<'a>,
    pub inner: T,
    pub close: RBracket<'a>,
}

impl<'a, T: Spanned + Node<'a>> Spanned for Bracket<'a, T> {
    fn span(&self) -> parmesan_common::Span {
        (self.open.span(), self.close.span()).into()
    }
}

impl<'a, T: Spanned + Node<'a>> Node<'a> for Bracket<'a, T> {
    fn parse(
        parser: &mut crate::parser::ParseStream<'a>,
    ) -> Result<Self, crate::error::ParseError<'a>>
    where
        Self: Sized,
    {
        let open = parser.step(LBracket::parse)?;
        let inner = parser.step(T::parse)?;
        let close = parser.step(RBracket::parse)?;
        Ok(Self { open, inner, close })
    }
}
#[test]
fn test_delimiter() {
    let mut parser = Parser::new("(abc)");
    let mut paren: Paren<Ident> = <Paren<Ident> as Node>::parse(&mut parser.stream()).unwrap();
    dbg!(&paren);
}
