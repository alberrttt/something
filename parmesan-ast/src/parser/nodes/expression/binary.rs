use std::error::Error;

use parmesan_common::Spanned;
use parmesan_dev_macros::{Parse, Spanned};

use crate::{
    error::ExpectedNode,
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
    pub(crate) left: Box<Expression<'a>>,
    pub(crate) operator: BinaryOperator<'a>,
    pub(crate) right: Box<Expression<'a>>,
}

impl<'a> Node<'a> for BinaryExpression<'a> {
    fn parse<'b: 'a>(
        parser: &'a mut crate::parser::Parser<'a>,
    ) -> Result<Self, crate::error::ParseError<'a>>
    where
        Self: Sized,
    {
        match Expression::parse(parser)? {
            Expression::BinaryExpression(bin) => Ok(bin),
            _ => Err(crate::error::ParseError::ExpectedNode(ExpectedNode {
                got: "Expression",
                expected: "BinaryExpression",
            })),
        }
    }
}

#[test]
fn test_bin() -> Result<(), Box<dyn Error>> {
    let mut lexer = Lexer::from("1+2");
    let tokens = lexer.lex();
    let mut parser = Parser {
        src: "1+2",
        tokens: &tokens,
        current: 0,
    };

    let bin: BinaryExpression = <BinaryExpression as Node>::parse(&mut parser).unwrap();
    dbg!(bin);
    Ok(())
}
