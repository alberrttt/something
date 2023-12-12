use std::{cell::UnsafeCell, error::Error};

use parm_common::Spanned;
mod precedence;
use crate::{
    lexer::token::{BinaryOperator, Identifier, Token},
    prelude::{ExpectedNode, Lexer, ParseError, ParseResult, ParseStream, Parser},
    source_file::PreparsedSourceFile,
    traits::{CreateDisplayNode, Node},
};

use self::{binary::BinaryExpression, number::Number, precedence::Precedence};

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
            Ok(Expression::Identifier(ident))
        }
        Token::Integer(_) => {
            let number = parser.step(|parser| Number::parse(parser).clone())?;
            Ok(Expression::Number(number))
        }
        Token::Float(_) => {
            let number = parser.step(|parser| Number::parse(parser).clone())?;
            Ok(Expression::Number(number))
        }
        _ => Err(ParseError::new(
            crate::error::ErrorKind::ExpectedNode(crate::error::ExpectedNode {
                got: token.lexeme(),
                expected: "an expression",
                location: parser.current,
            }),
            parser.tokens,
        )),
    }
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
impl CreateDisplayNode for Expression<'_> {
    fn create_display_node(&self) -> crate::parser::ast_displayer::DisplayNode {
        use Expression::*;
        match self {
            Identifier(ident) => ident.create_display_node(),
            Number(number) => number.create_display_node(),
            BinaryExpression(binary) => binary.create_display_node(),
        }
    }
}


#[test]
fn test_add() -> Result<(), Box<dyn Error>> {
    let src = "1 + a ** 2 + 3";
    let tokens = Lexer::from(src).lex();
    let mut parser = Parser {
        src,
        tokens,
        current: 0,
    };
    let preparsed = UnsafeCell::new(PreparsedSourceFile::new("./test".into(), src));
    let bin = <Expression as Node>::parse(&mut parser.stream(&preparsed)).unwrap();
    bin.create_display_node().display(0);
    Ok(())
}
