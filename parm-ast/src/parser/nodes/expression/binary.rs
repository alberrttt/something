use parm_common::Spanned;
use parm_dev_macros::Spanned;

use crate::{
    error::ExpectedNode,
    lexer::token::BinaryOperator,
    parser::ast_displayer::DisplayNode,
    prelude::{ErrorKind, ParseError, ParseResult},
    traits::{CreateDisplayNode, Node},
};

use super::Expression;

#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct BinaryExpression<'a> {
    pub left: Box<Expression<'a>>,
    pub operator: BinaryOperator<'a>,
    pub right: Box<Expression<'a>>,
}
impl CreateDisplayNode for BinaryExpression<'_> {
    fn create_display_node(&self) -> crate::parser::ast_displayer::DisplayNode {
        DisplayNode::new("BinaryExpression")
            .child(self.left.create_display_node().subtitle("Left: "))
            .child(self.operator.create_display_node().subtitle("Operator: "))
            .child(self.right.create_display_node().subtitle("Right: "))
    }
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
                    location: parser.current,
                }),
                parser.tokens,
            )),
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
