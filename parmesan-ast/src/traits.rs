use parmesan_common::Spanned;

use crate::{error::ParseError, parser::Parser};

pub trait Node<'a>: Spanned {
    fn parse(parser: &mut Parser<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized;
}
