use std::fmt::format;

use parm_dev_macros::Spanned;

use crate::ast::prelude::*;

use super::{expr, precedence::Precedence, Expression};

#[derive(Debug, Clone, PartialEq, Spanned, Tree)]
pub struct Group<'a> {
    pub paren: Paren<'a, Box<Expression<'a>>>,
}

impl<'a> Node<'a> for Group<'a> {
    fn parse(parser: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let paren: Paren<'a, Box<Expression<'a>>> = Paren::parse_manual(parser, |parser| {
            let expr = expr(parser, Precedence::Assignment)?;
            Ok(Box::new(expr))
        })?;
        Ok(Self { paren })
    }
}
