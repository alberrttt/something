use crate::lexer::token::SemiColon;

use super::expression::Expression;

pub mod expression_statement;
#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a> {
    Expression(Expression<'a>),
    ExpressionWithSemi((Expression<'a>, SemiColon<'a>)),
}
