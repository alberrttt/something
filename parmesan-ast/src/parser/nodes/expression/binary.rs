use std::error::Error;

use parmesan_common::Spanned;
use parmesan_dev_macros::{Parse, Spanned};

use crate::{
    lexer::{
        token::{Amper, BinaryOperator},
        Lexer,
    },
    parser::Parser,
    traits::Node,
};

use super::{number::Number, Expression};

#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct BinaryExpression<'a> {
    left: Box<Expression<'a>>,
    operator: BinaryOperator<'a>,
    right: Box<Expression<'a>>,
}

impl<'a> Node<'a> for BinaryExpression<'a> {
    fn parse<'b>(
        parser: &mut crate::parser::Parser<'a>,
    ) -> Result<Self, crate::error::ParseError<'a>>
    where
        Self: Sized,
    {
        let left: Expression = Node::parse(parser)?;
        let operator: BinaryOperator = Node::parse(parser)?;
        let right: Expression = Node::parse(parser)?;
        Ok(BinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
}
pub fn parse_binary_expression<'a>(
    parser: &mut Parser<'a>,
    expr: Option<Expression<'a>>,
) -> Result<BinaryExpression<'a>, crate::error::ParseError<'a>> {
    let left: Expression = match expr {
        Some(expr) => expr,
        None => Expression::parse(parser)?,
    };
    let operator: BinaryOperator = Node::parse(parser)?;
    let right: Expression = Node::parse(parser)?;
    Ok(BinaryExpression {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    })
}
#[test]
fn test_bin() -> Result<(), Box<dyn Error>> {
    let mut lexer = Lexer::from("1+2");
    let tokens = lexer.lex();
    let mut parser = Parser {
        src: "1+2*3+4",
        tokens: &tokens,
        current: 0,
    };

    let bin: BinaryExpression = <BinaryExpression as Node>::parse(&mut parser).unwrap();
    dbg!(bin);
    Ok(())
}
