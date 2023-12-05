use parm_dev_macros::Spanned;

use crate::prelude::*;

use super::expression::Expression;

pub mod expression_statement;
#[derive(Debug, Clone, PartialEq, Spanned)]
pub enum Statement<'a> {
    Expression(Expression<'a>),
    ExpressionWithSemi((Expression<'a>, SemiColon<'a>)),
}
impl<'a> Node<'a> for Statement<'a> {
    fn parse(parser: &mut crate::parser::ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let expr = parser.step(Expression::parse)?;

        let semi = parser.step(SemiColon::parse);
        Ok(Self::ExpressionWithSemi((expr, semi?)))
    }
}
