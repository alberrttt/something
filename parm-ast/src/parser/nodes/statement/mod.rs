use parm_dev_macros::Spanned;

use super::expression::Expression;
pub mod expression_statement;
pub mod use_stmt;
pub use use_stmt::*;
#[derive(Debug, Clone, PartialEq, Spanned)]
pub enum Statement<'a> {
    Expression(Expression<'a>),
    ExpressionWithSemi((Expression<'a>, SemiColon<'a>)),
    Item(Item<'a>),
    Let(LetStmt<'a>),
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
impl<'a> Statement<'a> {
    pub fn with_expression(parser: &mut ParseStream<'a>, expr: Expression<'a>) -> Self {
        let semi = parser.step(SemiColon::parse);
        match semi {
            Ok(semi) => Self::ExpressionWithSemi((expr, semi)),
            Err(_) => Self::Expression(expr),
        }
    }
}
