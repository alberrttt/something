pub mod binary;
pub mod block;
pub mod call;
pub mod group;
pub mod if_expr;
pub mod number;

use std::{cell::UnsafeCell, error::Error};

use parm_common::Spanned;
mod precedence;
use crate::ast::{
    lexer::token::{BinaryOperator, Identifier, Token},
    prelude::*,
    source_file::PreparsedSourceFile,
    traits::Node,
};

use self::{
    binary::BinaryExpression, block::Block, group::Group, number::Number, precedence::Precedence,
};

#[derive(Debug, Clone, PartialEq, Tree)]
pub enum Expression<'a> {
    Identifier(Identifier<'a>),
    Number(number::Number<'a>),
    StringLit(StringLiteral<'a>),
    BinaryExpression(binary::BinaryExpression<'a>),
    Group(Group<'a>),
    Call(Call<'a>),
    Block(Block<'a>),
    If(if_expr::IfExpr<'a>),
}

impl<'a> Node<'a> for Expression<'a> {
    fn parse(parser: &mut crate::ast::parser::ParseStream<'a>) -> ParseResult<'a, Self>
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
        let right = expr(parser, next_min_precedence)?;

        left = Expression::BinaryExpression(BinaryExpression {
            left: Box::new(left),
            operator: BinaryOperator::from(operator.clone()),
            right: Box::new(right),
        });
    }

    Ok(left)
}
pub fn atom<'a>(
    parse_stream: &mut crate::ast::parser::ParseStream<'a>,
) -> ParseResult<'a, Expression<'a>> {
    let token = parse_stream.peek()?;
    match token {
        Token::Identifier(_) => {
            let ident = parse_stream.step(|parser| Identifier::parse(parser).clone())?;
            let ident = Expression::Identifier(ident);
            if let Ok(Token::LParen(_)) = parse_stream.peek() {
                return Ok(Expression::Call(Call {
                    callee: Box::new(ident),
                    arguments: Call::args(parse_stream)?,
                }));
            }

            Ok(ident)
        }
        Token::If(_) => {
            let if_expr = parse_stream.step(if_expr::IfExpr::parse)?;
            Ok(Expression::If(if_expr))
        }
        Token::Integer(_) => {
            let number = parse_stream.step(|parser| Number::parse(parser).clone())?;
            Ok(Expression::Number(number))
        }
        Token::Float(_) => {
            let number = parse_stream.step(|parser| Number::parse(parser).clone())?;
            Ok(Expression::Number(number))
        }
        Token::LParen(_) => {
            let group = parse_stream.step(Group::parse)?;
            Ok(Expression::Group(group))
        }
        Token::LBrace(_) => {
            let block = parse_stream.step(block::Block::parse)?;
            Ok(Expression::Block(block))
        }
        Token::StringLiteral(str) => {
            parse_stream.advance()?;
            Ok(Expression::StringLit(str.clone()))
        }
        token => {
            parse_stream.panic = true;
            return ParseError::err(
                crate::ast::error::ErrorKind::ExpectedNode(crate::ast::error::ExpectedNode {
                    got: token.lexeme(),
                    expected: "an expression",
                    location: token.span(),
                }),
                parse_stream.tokens,
                parse_stream.src_file,
            );
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
            StringLit(string) => string.span(),
            If(if_expr) => if_expr.span(),
            Block(block) => block.span(),
        }
    }
}

#[test]
fn test_add() -> Result<(), Box<dyn Error>> {
    let (parser, preparsed) = parse!("1 + 2");
    let bin = <Expression as Node>::parse(&mut parser.stream(&preparsed)).unwrap();
    Ok(())
}

#[test]
fn test_pow() -> Result<(), Box<dyn Error>> {
    let (parser, preparsed) = parse!("1 + a**2+3");
    let bin = <Expression as Node>::parse(&mut parser.stream(&preparsed));
    match bin {
        Ok(bin) => {
            dbg!(&bin);
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
        Ok(bin) => {}
        Err(err) => eprint!("{}", err),
    }
    Ok(())
}
