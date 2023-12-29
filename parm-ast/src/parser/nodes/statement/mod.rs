use parm_dev_macros::Spanned;

use super::expression::Expression;
pub mod expression_statement;
pub mod use_stmt;
pub use use_stmt::*;
#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub enum Statement<'a> {
    Expression(Expression<'a>),
    ExpressionWithSemi(ExpressionWithSemi<'a>),
    Item(Item<'a>),
    Let(LetStmt<'a>),
}

#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub struct ExpressionWithSemi<'a> {
    pub expression: Expression<'a>,
    pub semi: SemiColon<'a>,
}
impl<'a> Node<'a> for Statement<'a> {
    fn parse(parser: &mut crate::parser::ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let expression = parser.step(Expression::parse)?;

        let semi = parser.step(SemiColon::parse);
        Ok(Self::ExpressionWithSemi(ExpressionWithSemi {
            expression,
            semi: semi?,
        }))
    }
}
impl<'a> Statement<'a> {
    pub fn with_expression(parser: &mut ParseStream<'a>, expression: Expression<'a>) -> Self {
        let semi = parser.step(SemiColon::parse);
        match semi {
            Ok(semi) => Self::ExpressionWithSemi(ExpressionWithSemi { expression, semi }),
            Err(_) => Self::Expression(expression),
        }
    }
}
