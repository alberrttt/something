use parmesan_common::Spanned;
use parmesan_dev_macros::{Parse, Spanned};

use crate::{
    lexer::token::{Amper, BinaryOperator},
    traits::Node,
};

use super::{number::Number, Expression};

#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct BinaryExpression<'a> {
    left: Box<Number<'a>>,
    operator: BinaryOperator<'a>,
    right: Box<Number<'a>>,
}

impl<'a> Node<'a> for BinaryExpression<'a> {
    fn parse<'b>(
        parser: &mut crate::parser::Parser<'a>,
    ) -> Result<Self, crate::error::ParseError<'a>>
    where
        Self: Sized,
    {
        let left: Number<'b> = Node::parse(parser)?;
        let operator: BinaryOperator = Node::parse(parser)?;
        let right: Number<'b> = Node::parse(parser)?;
        Ok(BinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
}
