pub mod binary;
pub mod block;
pub mod call;
pub mod expr_struct;
pub mod group;
pub mod if_expr;
pub mod number;

use std::{cell::UnsafeCell, error::Error, hint::unreachable_unchecked};

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

use super::path::{SimplePath, SimpleSegment};

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
    Path(SimplePath<'a>),
}
#[derive(Debug, Clone, PartialEq, Tree, Spanned)]
pub enum Boolean<'a> {
    True(TrueKw<'a>),
    False(FalseKw<'a>),
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
            Token::TrueKw(_) => Ok(Boolean::True(parse_stream.step(TrueKw::parse)?)),
            Token::FalseKw(_) => Ok(Boolean::False(parse_stream.step(FalseKw::parse)?)),
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
pub fn parsed_delimited<'a>(
    mut expr: Expression<'a>,
    parse_stream: &mut crate::parser::ParseStream<'a>,
) -> ParseResult<'a, Expression<'a>> {
    if let Ok(token) = parse_stream.peek() {
        parse_stream.panic = true;
        match token {
            Token::LParen(_) => {
                let call = parse_stream.step(call::Call::args)?;
                return Ok(Expression::Call(Call {
                    callee: Box::new(expr),
                    arguments: call,
                }));
            }
            Token::ColonColon(_) => {
                let Expression::Path(mut path) = expr else {
                    panic!()
                };
                let cc = ColonColon::parse(parse_stream)?;
                path.segments.push_punc(cc);
                let path = path.parse_more(parse_stream)?;
                let expr = {
                    let start = parse_stream.current;
                    let result = parsed_delimited(Expression::Path(path), parse_stream);
                    match result {
                        Ok(ok) => Ok(ok),
                        Err(err) => {
                            parse_stream.current = start;

                            Err(err)
                        }
                    }
                };
                return expr;
            }
            Token::LBrace(_) => {
                return Ok(Expression::StructExpression(StructExpression {
                    name: if let Expression::Path(path) = expr {
                        path
                    } else {
                        panic!()
                    },
                    body: parse_stream.step(|parser| parser.parse())?,
                }))
            }
            _ => {}
        }
        parse_stream.panic = false;
    }
    return Ok(expr);
}
pub fn atom<'a>(
    parse_stream: &mut crate::parser::ParseStream<'a>,
) -> ParseResult<'a, Expression<'a>> {
    let token = parse_stream.peek()?;
    let result = match token {
        Token::Identifier(_) => {
            let ident = parse_stream.step(|parser| Identifier::parse(parser).clone())?;
            if let Ok(token) = parse_stream.peek() {
                let path = SimplePath::from_ident(ident);
                let expr = parsed_delimited(Expression::Path(path), parse_stream);

                expr
            } else {
                Ok(Expression::Identifier(ident))
            }
        }
        Token::TrueKw(_) | Token::FalseKw(_) => {
            let boolean = parse_stream.step(|parser| Boolean::parse(parser).clone())?;
            Ok(Expression::Boolean(boolean))
        }
        Token::IfKw(_) => {
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
    };
    let delimited = parsed_delimited(result?, parse_stream);
    parse_stream.panic = false;

    delimited
}
impl Spanned for Expression<'_> {
    fn span(&self) -> parm_common::Span {
        use Expression::*;
        match self {
            Path(path) => path.span(),
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
