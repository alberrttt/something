use std::error::Error;

use parm_common::Spanned;
use parm_dev_macros::{Parse, Spanned};

use crate::{
    error::ExpectedNode,
    lexer::{
        token::{Amper, BinaryOperator},
        Lexer,
    },
    parser::Parser,
    prelude::{ErrorKind, ParseError, ParseResult},
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
    fn parse(parser: &mut crate::parser::ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        match Expression::parse(parser)? {
            Expression::BinaryExpression(bin) => Ok(bin),
            _ => Err(ParseError::new(
                ErrorKind::ExpectedNode(ExpectedNode {
                    got: "Expression",
                    expected: "BinaryExpression",
                }),
                parser.tokens,
            )),
        }
    }
}

#[test]
fn test_bin() -> Result<(), Box<dyn Error>> {
    let mut parser = Parser::new("1+2");

    let bin: BinaryExpression = <BinaryExpression as Node>::parse(&mut parser.stream()).unwrap();
    dbg!(bin);
    Ok(())
}
