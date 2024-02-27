use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub struct ReturnStatement<'a> {
    pub return_tkn: ReturnKw<'a>,
    pub expr: Expression<'a>,
    pub semi: SemiColon<'a>,
}
impl<'a> Node<'a> for ReturnStatement<'a> {
    fn parse(parser: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let return_tkn = parser.step(|parser| ReturnKw::parse(parser).clone())?;
        let expr = parser.step(|parser| Expression::parse(parser).clone())?;
        let semi = parser.step(|parser| SemiColon::parse(parser).clone())?;
        Ok(Self {
            return_tkn,
            expr,
            semi,
        })
    }
}
