pub mod function;
pub mod struct_dec;
pub mod trait_dec;
pub mod use_stmt;
use std::{fmt::Error, mem};

use crate::prelude::*;
use parm_common::Spanned;
use parm_dev_macros::Spanned;

use self::trait_dec::TraitDeclaration;

use super::comment::Comment;
#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub enum Item<'a> {
    Function(FunctionDeclaration<'a>),
    Use(UseStatement<'a>),
    Struct(StructDeclaration<'a>),
    Trait(TraitDeclaration<'a>),
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
        let res: Result<Item<'_>, Box<ParseError<'_>>> = match peeked {
            Token::StructKw(_) => {
                let struct_dec: StructDeclaration =
                    match <StructDeclaration as Node>::parse(parse_stream) {
                        Ok(ok) => ok,
                        Err(err) => return Some(Err(err)),
                    };
                Ok(Item::Struct(struct_dec))
            }
            Token::FnKw(_) => {
                let func: FunctionDeclaration =
                    match <FunctionDeclaration as Node>::parse(parse_stream) {
                        Ok(ok) => ok,
                        Err(err) => return Some(Err(err)),
                    };
                Ok(Item::Function(func))
            }

            Token::UseKw(_) => {
                let use_stmt: UseStatement = match <UseStatement as Node>::parse(parse_stream) {
                    Ok(ok) => ok,
                    Err(err) => return Some(Err(err)),
                };
                Ok(Item::Use(use_stmt))
            }

            Token::TraitKw(_) => {
                let trait_dec = match TraitDeclaration::parse(parse_stream) {
                    Ok(ok) => ok,
                    Err(err) => return Some(Err(err)),
                };

                Ok(Item::Trait(trait_dec))
            }
            _ => return None,
        };
        Some(res)
    }
}
