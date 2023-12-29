use std::{fmt::Error, mem};

use crate::prelude::*;
use parm_common::Spanned;
use parm_dev_macros::Spanned;

use super::comment::Comment;

#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub enum Item<'a> {
    Variable(LetStmt<'a>),
    Function(Function<'a>),
    Use(UseStatement<'a>),
    Struct(Struct<'a>),
}

impl<'a> Node<'a> for Item<'a> {
    fn parse(parse_stream: &mut super::ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let attributes: ParseResult<'_, Vec<Attribute<'_>>> =
            parse_stream.step(Vec::<Attribute>::parse);
        for error in mem::take(&mut parse_stream.errors) {
            if let ErrorKind::ExpectedToken(expected) = &error.kind {
                match &expected.expected {
                    Token::Octothorpe(_) => {}
                    _ => {
                        eprintln!("{}", error);
                    }
                }
            }
        }
        if let Ok(mut attributes) = attributes {
            parse_stream.attributes.append(&mut attributes);
        }
        let peeked = parse_stream.peek()?;

        match peeked {
            Token::Let(_) => match <LetStmt as Node>::parse(parse_stream) {
                Ok(ok) => return Ok(Item::Variable(ok)),
                Err(err) => {
                    parse_stream.panic = true;
                    return Err(err);
                }
            },
            Token::StructKeyword(_) => {
                let struct_dec: Struct = <Struct as Node>::parse(parse_stream)?;
                return Ok(Item::Struct(struct_dec));
            }
            Token::FnKeyword(_) => {
                let func: Function = <Function as Node>::parse(parse_stream)?;
                return Ok(Item::Function(func));
            }

            Token::Use(_) => {
                let use_stmt: UseStatement = <UseStatement as Node>::parse(parse_stream)?;
                return Ok(Item::Use(use_stmt));
            }
            _ => {}
        }
        parse_stream.panic = true;
        ParseError::err(
            crate::error::ErrorKind::ExpectedNode(crate::error::ExpectedNode {
                got: parse_stream.peek()?.lexeme(),
                expected: "an item",
                location: parse_stream.current,
            }),
            parse_stream.tokens,
            parse_stream.src_file,
        )
    }
}
#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct ReturnStatement<'a> {
    pub return_tkn: Return<'a>,
    pub expr: Expression<'a>,
    pub semi: SemiColon<'a>,
}
impl<'a> Node<'a> for ReturnStatement<'a> {
    fn parse(parser: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let return_tkn = parser.step(|parser| Return::parse(parser).clone())?;
        let expr = parser.step(|parser| Expression::parse(parser).clone())?;
        let semi = parser.step(|parser| SemiColon::parse(parser).clone())?;
        Ok(Self {
            return_tkn,
            expr,
            semi,
        })
    }
}
// #[test]
// fn test_var() {
//     let pre = PreparsedSourceFile::new("test".into(), "let x = 1;");
//     let mut parser = Parser::new("let x = 1;");
//     let var: Item = <Item as Node>::parse(&mut parser.stream()).unwrap();
//     dbg!(var);
// }

// #[test]
// fn test_fn() {
//     let mut parser = Parser::new("fn x() {}");
//     let var: Item = <Item as Node>::parse(&mut parser.stream()).unwrap();
//     dbg!(var);
// }
