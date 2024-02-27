use crate::prelude::*;

use super::{Brace, Identifier};

#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub struct TraitDeclaration<'a> {
    pub trait_kw: crate::lexer::token::TraitKw<'a>,
    pub name: Identifier<'a>,
    pub body: Brace<'a, crate::parser::nodes::item::trait_dec::TraitBody<'a>>,
}
impl<'a> Node<'a, ParseResult<'a, Self>> for TraitDeclaration<'a> {
    fn parse(parse_stream: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let trait_kw = parse_stream.step(crate::lexer::token::TraitKw::parse)?;
        parse_stream.panic = true;
        let name = parse_stream.step(Identifier::parse)?;
        let body = parse_stream.step(Brace::parse)?;
        Ok(Self {
            trait_kw,
            name,
            body,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub struct TraitBody<'a> {
    pub members: Vec<TraitBodyMember<'a>>,
}
impl<'a> Node<'a, ParseResult<'a, Self>> for TraitBody<'a> {
    fn parse(parse_stream: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        Ok(Self { members: vec![] })
    }
}

#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub enum TraitBodyMember<'a> {
    Function(FunctionDeclaration<'a>),
}
