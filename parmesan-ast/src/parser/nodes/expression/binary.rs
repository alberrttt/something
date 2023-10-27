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
        let left: Number = Node::parse(parser)?;
        let operator: BinaryOperator = Node::parse(parser)?;
        let right: Number = Node::parse(parser)?;
        Ok(BinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
}
#[test]
fn test_bin() ->  Result<(), Box<dyn Error>> {
    let mut lexer = Lexer::from("1+2");
    let tokens = lexer.lex();
    let mut parser = Parser {
        src: "1+2",
        tokens: &tokens,
        current: 0,
    };

    let bin: BinaryExpression = Node::parse(&mut parser).unwrap();
    dbg!(bin);
    Ok(())
}
