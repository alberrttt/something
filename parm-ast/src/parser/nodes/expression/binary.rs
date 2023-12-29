use parm_common::Spanned;
use parm_dev_macros::{Spanned, Tree};

use super::Expression;
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub struct BinaryExpression<'a> {
    pub left: Box<Expression<'a>>,
    pub operator: BinaryOperator<'a>,
    pub right: Box<Expression<'a>>,
}

impl<'a> Node<'a> for BinaryExpression<'a> {
    fn parse(parse_stream: &mut crate::parser::ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        match Expression::parse(parse_stream)? {
            Expression::BinaryExpression(bin) => Ok(bin),
            _ => ParseError::err(
                ErrorKind::ExpectedNode(ExpectedNode {
                    got: "Expression",
                    expected: "BinaryExpression",
                    location: parse_stream.current,
                }),
                parse_stream.tokens,
                parse_stream.src_file,
            ),
        }
    }
}

// #[test]
// fn test_bin() -> Result<(), Box<dyn Error>> {
//     let mut parser = Parser::new("1+2");

//     let bin: BinaryExpression = <BinaryExpression as Node>::parse(&mut parser.stream()).unwrap();
//     dbg!(bin);
//     Ok(())
// }
