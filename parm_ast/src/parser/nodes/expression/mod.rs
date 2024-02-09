pub mod binary;
pub mod block;
pub mod call;
pub mod expr_struct;
pub mod group;
pub mod if_expr;
pub mod number;

use std::{cell::UnsafeCell, error::Error};

use parm_common::Spanned;
mod precedence;
use crate::{
    lexer::token::{BinaryOperator, Identifier, Token},
    prelude::*,
    source_file::PreparsedSourceFile,
    traits::Node,
};

use self::{
    binary::BinaryExpression, block::Block, expr_struct::StructExpression, group::Group,
    number::Number, precedence::Precedence,
};

#[derive(Debug, Clone, PartialEq, Tree)]
pub enum Expression<'a> {
    Identifier(Identifier<'a>),
    StructExpression(StructExpression<'a>),
    Number(number::Number<'a>),
    StringLit(StringLiteral<'a>),
    BinaryExpression(binary::BinaryExpression<'a>),
    Group(Group<'a>),
    Call(Call<'a>),
    Block(Block<'a>),
    If(if_expr::IfExpr<'a>),
    Boolean(Boolean<'a>),
}
#[derive(Debug, Clone, PartialEq, Tree, Spanned)]
pub enum Boolean<'a> {
    True(True<'a>),
    False(False<'a>),
}
impl<'a> Boolean<'a> {
    pub fn value(&self) -> bool {
        match self {
            Boolean::True(_) => true,
            Boolean::False(_) => false,
        }
    }
}
impl<'a> Node<'a> for Boolean<'a> {
    fn parse(parse_stream: &mut ParseStream<'a>) -> Result<Self, Box<ParseError<'a>>>
    where
        Self: Sized,
    {
        let token = parse_stream.peek()?;
        match token {
            Token::True(_) => Ok(Boolean::True(parse_stream.step(True::parse)?)),
            Token::False(_) => Ok(Boolean::False(parse_stream.step(False::parse)?)),
            _ => ParseError::err(
                ErrorKind::ExpectedNode(ExpectedNode {
                    expected: "a boolean",
                    got: token.lexeme(),
                    location: token.span(),
                }),
                parse_stream.tokens,
                parse_stream.src_file,
            ),
        }
    }
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
    parse_stream: &mut crate::parser::ParseStream<'a>,
) -> ParseResult<'a, Expression<'a>> {
    let token = parse_stream.peek()?;
    match token {
        Token::Identifier(_) => {
            let ident = parse_stream.step(|parser| Identifier::parse(parser).clone())?;
            let ident = Expression::Identifier(ident);
            if let Ok(token) = parse_stream.peek() {
                parse_stream.panic = true;
                match token {
                    Token::LParen(_) => {
                        let call = parse_stream.step(call::Call::args)?;
                        return Ok(Expression::Call(Call {
                            callee: Box::new(ident),
                            arguments: call,
                        }));
                    }
                    Token::True(_) | Token::False(_) => {
                        let boolean = parse_stream.step(|parser| Boolean::parse(parser).clone())?;
                        return Ok(Expression::Boolean(boolean));
                    }
                    Token::LBrace(_) => {
                        return Ok(Expression::StructExpression(StructExpression {
                            ident: match ident {
                                Expression::Identifier(ident) => ident,
                                _ => unreachable!(),
                            },
                            body: parse_stream.step(|parser| parser.parse())?,
                        }))
                    }
                    _ => {}
                }
                parse_stream.panic = false;
            }

            Ok(ident)
        }
        Token::True(_) | Token::False(_) => {
            let boolean = parse_stream.step(|parser| Boolean::parse(parser).clone())?;
            Ok(Expression::Boolean(boolean))
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
                crate::error::ErrorKind::ExpectedNode(crate::error::ExpectedNode {
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
            StructExpression(struct_expr) => struct_expr.span(),
            Identifier(ident) => ident.span(),
            Number(number) => number.span(),
            BinaryExpression(binary) => binary.span(),
            Group(group) => group.span(),
            Call(call) => call.span(),
            StringLit(string) => string.span(),
            If(if_expr) => if_expr.span(),
            Block(block) => block.span(),
            Boolean(boolean) => boolean.span(),
        }
    }
}
