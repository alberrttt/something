use parmesan_common::Spanned;
use parmesan_dev_macros::Spanned;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct Variable<'a> {
    pub let_tkn: Let<'a>,
    pub ident: Ident<'a>,
    pub initializer: Option<Initializer<'a>>,
    pub semi: SemiColon<'a>,
}
#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct Initializer<'a> {
    pub eq: Eq<'a>,
    pub expr: Expression<'a>,
}
impl<'a> Node<'a> for Variable<'a> {
    fn parse(
        parser: &mut crate::parser::ParseStream<'a>,
    ) -> Result<Self, crate::error::ParseError<'a>>
    where
        Self: Sized,
    {
        let let_tkn = parser.step(Let::parse)?;
        let ident = parser.step(Ident::parse)?;
        let initializer = parser.step(|parser| {
            let eq = parser.step(Eq::parse)?;
            let expr = parser.step(Expression::parse)?;
            Ok(Initializer { eq, expr })
        });
        let semi = parser.step(SemiColon::parse)?;
        Ok(Self {
            let_tkn,
            ident,
            initializer: initializer.ok(),
            semi,
        })
    }
}
