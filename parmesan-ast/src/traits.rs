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

impl<'a, T: Node<'a> + Spanned> Node<'a> for Vec<T> {
    fn parse(parser: &mut ParseStream<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        let mut vec = Vec::new();
        loop {
            if parser.at_end() {
                break;
            }
            let value = T::parse(parser)?;
            vec.push(value);
        }
        Ok(vec)
    }
}
