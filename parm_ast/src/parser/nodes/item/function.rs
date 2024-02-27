use std::mem;

use crate::{
    parser::nodes::expression::block::{self, Block},
    prelude::*,
};
use parm_common::Spanned;
use parm_dev_macros::Spanned;
#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub struct Param<'a> {
    pub name: Identifier<'a>,
    pub annotation: TypeAnnotation<'a>,
}
impl<'a> Node<'a> for Param<'a> {
    fn parse(parser: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let name = parser.step(|parser| Identifier::parse(parser).clone())?;
        let annotation = parser.step(|parser| TypeAnnotation::parse(parser).clone())?;
        Ok(Self { name, annotation })
    }
}
#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub struct ReturnType<'a> {
    pub arrow: RightArrow<'a>,
    pub ret_type: TypeExpression<'a>,
}
impl<'a> Node<'a> for ReturnType<'a> {
    fn parse(parser: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let arrow = parser.step(|parser| RightArrow::parse(parser).clone());
        match arrow {
            Ok(arrow) => {
                let ret_type = parser.step(|parser| TypeExpression::parse(parser).clone())?;
                Ok(Self { arrow, ret_type })
            }
            Err(err) => return Err(err),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub struct FunctionDeclaration<'a> {
    pub attributes: Vec<Attribute<'a>>,
    pub fn_tkn: FnKw<'a>,
    pub name: Identifier<'a>,
    pub params: Paren<'a, Punctuated<Param<'a>, Comma<'a>>>,
    pub body: Block<'a>,
    pub ret_type: Option<ReturnType<'a>>,
}
impl<'a> Node<'a> for FunctionDeclaration<'a> {
    fn parse(parser: &mut crate::parser::ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let fn_token = parser.step(|parser| FnKw::parse(parser).clone())?;
        let name = Identifier::parse(parser)?;
        let params: Paren<'_, Punctuated<Param<'_>, Comma<'_>>> = parser.step(|parser| {
            Paren::parse_manual(parser, |parser| {
                let punct = Punctuated::default();
                punct.parse_terminated(parser)
            })
        })?;
        let body: Block<'_> = parser.step(Block::parse)?;
        let ret_type = parser.step(ReturnType::parse).ok();
        Ok(Self {
            attributes: mem::take(&mut parser.attributes),
            fn_tkn: fn_token,
            name,
            params,
            body,
            ret_type,
        })
    }
}

// #[test]
// fn test_fn() {
//     let input = "fn foo(hello) {
//         let x = 5;
//         let y = 6;
//         x + 2;
//     } -> wassup::bejing<foo,bar>::icecream";
//     let mut parser = Parser::new(input);
//     let result = Function::parse(&mut parser.stream()).unwrap();
//     dbg!(result);
// }
