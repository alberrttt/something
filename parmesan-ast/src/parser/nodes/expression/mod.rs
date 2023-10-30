use parmesan_common::Spanned;

use crate::{error::EndOfTokens, lexer::token, parser, traits::Node};

use self::{binary::parse_binary_expression, ident::Identifier, number::Number};

pub mod binary;
pub mod ident;
pub mod number;
#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Identifier(ident::Identifier<'a>),
    Number(number::Number<'a>),
    BinaryExpression(binary::BinaryExpression<'a>),
}
impl<'a> Node<'a> for Expression<'a> {
    fn parse(parser: &mut crate::parser::Parser<'a>) -> Result<Self, crate::error::ParseError<'a>>
    where
        Self: Sized,
    {
        parse_term(parser, None)
    }
}
fn parse_term<'a>(
    parser: &mut crate::parser::Parser<'a>,
    expr: Option<Expression<'a>>,
) -> Result<Expression<'a>, crate::error::ParseError<'a>> {
    let peek = parser.peek();
    let expr = match expr {
        Some(expr) => Ok(expr),
        None => Err(crate::error::ParseError::EndOfTokens(
            crate::error::EndOfTokens {},
        )),
    };
    let Ok(peeked) = peek else {
        return expr;
    };

    match peeked {
        token::Token::Plus(plus) => match expr {
            Ok(expr) => Ok(Expression::BinaryExpression(parse_binary_expression(
                parser,
                Some(expr),
            )?)),
            Err(err) => Err(err),
        },
        _ => expr,
    }
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
