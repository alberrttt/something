use std::error::Error;

use parmesan_common::Spanned;
mod precedence;
use crate::{
    error::{EndOfTokens, ParseError},
    lexer::{
        token::{self, BinaryOperator, Ident, Token},
        Lexer,
    },
    parser::{self, Parser},
    traits::Node,
};

use self::{binary::BinaryExpression, number::Number};

pub mod binary;
pub mod literal;
pub mod number;
#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Identifier(Ident<'a>),
    Number(number::Number<'a>),
    BinaryExpression(binary::BinaryExpression<'a>),
}
impl<'a> Node<'a> for Expression<'a> {
    fn parse(parser: &mut crate::parser::Parser<'a>) -> Result<Self, crate::error::ParseError<'a>>
    where
        Self: Sized,
    {
        parse_expression(parser)
    }
}
fn parse_unit<'a>(
    parser: &mut crate::parser::Parser<'a>,
) -> Result<Expression<'a>, crate::error::ParseError<'a>> {
    let peeked = parser.peek()?;

    match peeked {
        Token::Integer(_) | Token::Float(_) => {
            parser.advance()?;
            Ok(Expression::Number(Number::from(peeked)))
        }
        _ => Err(crate::error::ParseError::EndOfTokens(EndOfTokens {})),
    }
}
fn parse_expression<'a>(parser: &mut Parser<'a>) -> Result<Expression<'a>, ParseError<'a>> {
    let mut left = parse_unit(parser)?;
    while match parser.peek() {
        Err(_) => false,
        Ok(token) => matches!(token, Token::Plus(_) | Token::Minus(_)),
    } {
        dbg!(parser.peek()?);
        let operator: BinaryOperator = BinaryOperator::parse(parser)?;
        let right = parse_term(parser)?;
        left = Expression::BinaryExpression(BinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        });
    }
    dbg!(&left);
    Ok(left)
}
fn parse_term<'a>(parser: &mut Parser<'a>) -> Result<Expression<'a>, ParseError<'a>> {
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
    fn span(&self) -> parmesan_common::Span {
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

    let bin = <Expression as Node>::parse(&mut parser).unwrap();
    dbg!(bin);
    Ok(())
}
