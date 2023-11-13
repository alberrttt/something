use parmesan_common::Spanned;

use crate::{
    error::ParseError,
    parser::{parse_stream::ParseStream, Parser},
};

pub trait Node<'a>: Spanned {
    fn parse(parser: &mut ParseStream<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized;
}
