use std::error::Error;

use parm_common::Spanned;
mod precedence;
use crate::{
    error::{EndOfTokens, ErrorKind},
    lexer::{
        token::{self, BinaryOperator, Identifier, Token},
        Lexer,
    },
    parser::{self, Parser},
    prelude::{ExpectedNode, ParseError, ParseResult},
    traits::Node,
};

use self::{binary::BinaryExpression, number::Number};

pub mod binary;
pub mod literal;
pub mod number;
#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Identifier(Identifier<'a>),
    Number(number::Number<'a>),
    BinaryExpression(binary::BinaryExpression<'a>),
}
impl<'a> Node<'a> for Expression<'a> {
    fn parse(parser: &mut crate::parser::ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        parse_expression(parser)
    }
}
pub fn parse_unit<'a>(
    parser: &mut crate::parser::ParseStream<'a>,
) -> ParseResult<'a, Expression<'a>> {
    let peeked = parser.peek()?;

    match peeked {
        Token::Integer(_) | Token::Float(_) => {
            parser.advance()?;
            Ok(Expression::Number(Number::from(peeked)))
        }
        Token::Identifier(_) => Ok(Expression::Identifier(Identifier::parse(parser)?)),
        token => Err(ParseError::new(
            crate::error::ErrorKind::ExpectedNode(ExpectedNode {
                expected: "expression",
                got: token.lexeme(),
            }),
            parser.tokens,
        )),
    }
}
fn parse_expression<'a>(
    parser: &mut crate::parser::ParseStream<'a>,
) -> ParseResult<'a, Expression<'a>> {
    let mut left = parse_unit(parser)?;
    while match parser.peek() {
        Err(_) => false,
        Ok(token) => matches!(token, Token::Plus(_) | Token::Minus(_)),
    } {
        let operator: BinaryOperator = BinaryOperator::parse(parser)?;
        let right = parse_term(parser)?;
        left = Expression::BinaryExpression(BinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        });
    }
    Ok(left)
}
fn parse_term<'a>(parser: &mut crate::parser::ParseStream<'a>) -> ParseResult<'a, Expression<'a>> {
    let mut left: Expression<'_> = parse_unit(parser)?;
    while match parser.peek() {
        Err(_) => false,
        Ok(token) => matches!(token, Token::Star(_) | Token::Slash(_)),
    } {
        dbg!(parser.peek()?);
        let operator: BinaryOperator = BinaryOperator::parse(parser)?;
        let right = parse_unit(parser)?;
        left = Expression::BinaryExpression(BinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        });
    }

    Ok(left)
}

impl Spanned for Expression<'_> {
    fn span(&self) -> parm_common::Span {
        use Expression::*;
        match self {
            Identifier(ident) => ident.span(),
            Number(number) => number.span(),
            BinaryExpression(binary) => binary.span(),
        }
    }
}
#[test]
fn test_expr() -> Result<(), Box<dyn Error>> {
    let src = "1+2*3+4";
    let mut parser = Parser::new(src);

    let bin = <Expression as Node>::parse(&mut parser.stream()).unwrap();
    dbg!(bin);
    Ok(())
}
#[test]
fn test_add() -> Result<(), Box<dyn Error>> {
    let src = "a*2";
    let mut parser = Parser::new(src);

    let bin = <Expression as Node>::parse(&mut parser.stream()).unwrap();
    dbg!(bin);
    Ok(())
}
