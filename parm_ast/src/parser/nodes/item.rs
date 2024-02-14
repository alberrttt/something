use std::{fmt::Error, mem};

use crate::prelude::*;
use parm_common::Spanned;
use parm_dev_macros::Spanned;

use super::comment::Comment;
#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub enum Item<'a> {
    LetStatement(LetStatement<'a>),
    Function(FunctionDeclaration<'a>),
    Use(UseStatement<'a>),
    Struct(StructDeclaration<'a>),
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
        let mut peek_n = 0;
        let peeked = parse_stream.peek_n(peek_n)?;
        if let Token::Pub(_) = peeked {
            peek_n += 1;
            return Self::peek_and_parse(parse_stream, peek_n).unwrap();
        }
        if let Some(result) = Self::peek_and_parse(parse_stream, peek_n) {
            return result;
        }
        parse_stream.panic = true;
        ParseError::err(
            crate::error::ErrorKind::ExpectedNode(crate::error::ExpectedNode {
                got: parse_stream.peek()?.lexeme(),
                expected: "an item",
                location: peeked.span(),
            }),
            parse_stream.tokens,
            parse_stream.src_file,
        )
    }
}
impl<'a> Item<'a> {
    fn peek_and_parse(
        parse_stream: &mut super::ParseStream<'a>,
        peek: usize,
    ) -> Option<ParseResult<'a, Item<'a>>> {
        let peeked = match parse_stream.peek_n(peek) {
            Ok(ok) => ok,
            Err(err) => return None,
        };
        let res = match peeked {
            Token::Let(_) => match <LetStatement as Node>::parse(parse_stream) {
                Ok(ok) => Ok(Item::LetStatement(ok)),
                Err(err) => {
                    parse_stream.panic = true;
                    Err(err)
                }
            },

            Token::StructKeyword(_) => {
                let struct_dec: StructDeclaration =
                    match <StructDeclaration as Node>::parse(parse_stream) {
                        Ok(ok) => ok,
                        Err(err) => return Some(Err(err)),
                    };
                Ok(Item::Struct(struct_dec))
            }
            Token::FnKeyword(_) => {
                let func: FunctionDeclaration =
                    match <FunctionDeclaration as Node>::parse(parse_stream) {
                        Ok(ok) => ok,
                        Err(err) => return Some(Err(err)),
                    };
                Ok(Item::Function(func))
            }

            Token::Use(_) => {
                let use_stmt: UseStatement = match <UseStatement as Node>::parse(parse_stream) {
                    Ok(ok) => ok,
                    Err(err) => return Some(Err(err)),
                };
                Ok(Item::Use(use_stmt))
            }
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
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
