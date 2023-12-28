use std::{cell::UnsafeCell, error::Error};

use parm_common::Spanned;
mod precedence;
use crate::{
    lexer::token::{BinaryOperator, Identifier, Token},
    prelude::*,
    source_file::PreparsedSourceFile,
    traits::{CreateDisplayNode, Node},
};

use self::{binary::BinaryExpression, group::Group, number::Number, precedence::Precedence};

use super::statement::parse;
pub mod binary;
pub mod call;
pub mod group;
pub mod literal;
pub mod number;
#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Identifier(Identifier<'a>),
    Number(number::Number<'a>),
    BinaryExpression(binary::BinaryExpression<'a>),
    Group(Group<'a>),
    Call(Call<'a>),
}

impl<'a> Node<'a> for Expression<'a> {
    fn parse(parser: &mut crate::parser::ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        expr(parser, Precedence::Assignment)
    }
}
pub fn expr<'a>(
    parser: &mut ParseStream<'a>,
    min_precedence: Precedence,
) -> ParseResult<'a, Expression<'a>> {
    let mut left = atom(parser)?;
    loop {
        let Ok(next) = parser.peek() else { break };
        let precedence = Precedence::from(next);
        if (precedence < min_precedence) || !BinaryOperator::token_is_member(next) {
            break;
        }
        let operator = parser.advance()?;
        let next_min_precedence = precedence.increment();
        let mut right = expr(parser, next_min_precedence)?;

        left = Expression::BinaryExpression(BinaryExpression {
            left: Box::new(left),
            operator: BinaryOperator::from(operator.clone()),
            right: Box::new(right),
        });
    }

    Ok(left)
}
pub fn atom<'a>(parser: &mut crate::parser::ParseStream<'a>) -> ParseResult<'a, Expression<'a>> {
    let token = parser.peek()?;
    match token {
        Token::Identifier(_) => {
            let ident = parser.step(|parser| Identifier::parse(parser).clone())?;
            let ident = Expression::Identifier(ident);
            if let Ok(Token::LParen(_)) = parser.peek() {
                return Ok(Expression::Call(Call {
                    callee: Box::new(ident),
                    arguments: Call::args(parser)?,
                }));
            }

            Ok(ident)
        }
        Token::Integer(_) => {
            let number = parser.step(|parser| Number::parse(parser).clone())?;
            Ok(Expression::Number(number))
        }
        Token::Float(_) => {
            let number = parser.step(|parser| Number::parse(parser).clone())?;
            Ok(Expression::Number(number))
        }
        Token::LParen(_) => {
            let group = parser.step(Group::parse)?;
            Ok(Expression::Group(group))
        }
        _ => {
            parser.panic = true;
            return Err(ParseError::new(
                crate::error::ErrorKind::ExpectedNode(crate::error::ExpectedNode {
                    got: token.lexeme(),
                    expected: "an expression",
                    location: parser.current,
                }),
                parser.tokens,
            ));
        }
    }
}
impl Spanned for Expression<'_> {
    fn span(&self) -> parm_common::Span {
        use Expression::*;
        match self {
            Identifier(ident) => ident.span(),
            Number(number) => number.span(),
            BinaryExpression(binary) => binary.span(),
            Group(group) => group.span(),
            Call(call) => call.span(),
        }
    }
}
impl CreateDisplayNode for Expression<'_> {
    fn create_display_node(&self) -> crate::parser::ast_displayer::DisplayNode {
        use Expression::*;
        match self {
            Identifier(ident) => ident.create_display_node(),
            Number(number) => number.create_display_node(),
            BinaryExpression(binary) => binary.create_display_node(),
            Group(group) => group.create_display_node(),
            Call(call) => todo!(),
        }
    }
}

#[test]
fn test_add() -> Result<(), Box<dyn Error>> {
    let (parser, preparsed) = parse!("1 + 2");
    let bin = <Expression as Node>::parse(&mut parser.stream(&preparsed)).unwrap();
    bin.create_display_node().display(0);
    Ok(())
}

#[test]
fn test_pow() -> Result<(), Box<dyn Error>> {
    let (parser, preparsed) = parse!("1 + a**2+3");
    let bin = <Expression as Node>::parse(&mut parser.stream(&preparsed));
    match bin {
        Ok(bin) => {
            dbg!(&bin);
            bin.create_display_node().display(0);
        }
        Err(err) => {
            eprint!("{}", err);
            panic!()
        }
    }
    Ok(())
}
#[test]
fn test_group() -> Result<(), Box<dyn Error>> {
    let (parser, preparsed) = parse!("a**(1 + 2)");
    let bin = <Expression as Node>::parse(&mut parser.stream(&preparsed));
    match bin {
        Ok(bin) => {
            bin.create_display_node().display(0);
        }
        Err(err) => eprint!("{}", err),
    }
    Ok(())
}
