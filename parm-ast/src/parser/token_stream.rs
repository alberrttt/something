use parm_common::Spanned;

use crate::prelude::{Node, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct TokenStream<'a>(&'a [Token<'a>]);

impl<'a> Spanned for TokenStream<'a> {
    fn span(&self) -> parm_common::Span {
        self.0
            .first()
            .unwrap()
            .span()
            .join(self.0.last().unwrap().span())
    }
}

impl<'a> Node<'a> for TokenStream<'a> {
    fn parse(
        parser: &mut crate::prelude::ParseStream<'a>,
    ) -> Result<Self, crate::prelude::ParseError<'a>>
    where
        Self: Sized,
    {
        parser.current = parser.tokens.len();
        Ok(Self(parser.tokens))
    }
}
