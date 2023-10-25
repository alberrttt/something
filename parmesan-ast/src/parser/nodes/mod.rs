use parmesan_common::Spanned;

pub mod expression;
pub mod statement;
use expression::*;
#[derive(Debug, Clone)]
pub enum Node<'a> {
    Expression(expression::Expression<'a>),
}
