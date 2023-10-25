use parmesan_common::Spanned;

use crate::{lexer::token::SemiColon, parser::nodes::expression::Expression, traits::Parse};

#[derive(Debug, Clone)]
pub struct ExpressionStatement<'a> {
    expression: Expression<'a>,
    semi_colon: SemiColon<'a>,
}
impl<'a> Spanned for ExpressionStatement<'a> {
    fn span(&self) -> parmesan_common::Span {
        (self.expression.span(), self.semi_colon.span).into()
    }
}
impl<'a> Parse for ExpressionStatement<'a> {
    fn parse<'src>(
        parser: &mut crate::parser::Parser<'src>,
    ) -> Result<Self, Box<dyn crate::error::ParseError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
