use parm_common::Spanned;

use crate::{
    error::ErrorKind,
    parser::{parse_stream::ParseStream, Parser},
    prelude::{ParseError, ParseResult, Token},
};

pub trait Node<'a, Output = ParseResult<'a, Self>>: Spanned {
    fn parse(parser: &mut ParseStream<'a>) -> Output
    where
        Self: Sized;
    fn to_tokens<'b>(&self) -> &'b [Token<'a>] {
        todo!()
    }
}
impl<'a, T: Node<'a> + Spanned> Node<'a, (Self, Vec<ParseError<'a>>)> for Vec<T> {
    fn parse(parser: &mut ParseStream<'a>) -> (Self, Vec<ParseError<'a>>)
    where
        Self: Sized,
    {
        let mut vec = Vec::new();
        let mut errors = Vec::new();
        loop {
            if parser.at_end() {
                break;
            }
            match T::parse(parser) {
                Ok(ok) => vec.push(ok),
                Err(err) => {
                    errors.push(err);
                    break;
                }
            }
        }
        (vec, errors)
    }
}
impl<'a, T: Node<'a> + Spanned> Node<'a, ParseResult<'a, Self>> for Vec<T> {
    fn parse(parser: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let (result, errors) = <Vec<T> as Node<'a, (Self, Vec<ParseError<'a>>)>>::parse(parser);
        for error in errors {
            eprintln!("{}", error);
        }
        Ok(result)
    }
}
