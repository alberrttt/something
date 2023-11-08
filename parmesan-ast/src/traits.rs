use parmesan_common::Spanned;

use crate::{error::ParseError, parser::Parser};

pub trait Node<'a>: Spanned {
    fn parse<'b: 'a>(parser: &'b mut Parser<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized;
}
