use parmesan_common::Spanned;
use parmesan_dev_macros::Spanned;

use crate::{lexer::token::BinaryOperator, parser::nodes::Node};

use super::Expression;

#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct BinaryExpression<'a> {
    left: Box<Expression<'a>>,
    operator: BinaryOperator<'a>,
    right: Box<Expression<'a>>,
}
