use std::marker::PhantomData;

use parm_common::Spanned;

use crate::ast::{
    parser::parse_stream::ParseStream,
    prelude::{ParseError, ParseResult, Token},
};

pub trait EscapedText {
    fn escaped_text(&self) -> String;
}
pub trait Node<'a, Output = ParseResult<'a, Self>>: Spanned {
    fn parse(parse_stream: &mut ParseStream<'a>) -> Output
    where
        Self: Sized;
}
impl<'a, T: Node<'a> + Spanned + std::fmt::Debug> Node<'a, (Self, Vec<ParseError<'a>>)> for Vec<T> {
    fn parse(parser: &mut ParseStream<'a>) -> (Self, Vec<ParseError<'a>>)
    where
        Self: Sized,
    {
        let mut vec = Vec::new();
        let mut errors: Vec<ParseError<'_>> = Vec::new();

        loop {
            if parser.at_end() {
                break;
            }
            match T::parse(parser) {
                Ok(ok) => vec.push(ok),
                Err(err) => {
                    errors.push(*err);
                    break;
                }
            }
        }

        (vec, errors)
    }
}
impl<'a, T: Node<'a> + Spanned + std::fmt::Debug> Node<'a> for Vec<T> {
    fn parse(parser: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let (result, errors) = <Vec<T> as Node<'a, (Self, Vec<ParseError<'a>>)>>::parse(parser);
        for error in errors {
            parser.errors.push(error)
        }
        Ok(result)
    }
}
impl<'a, T: Node<'a>> Node<'a> for Box<T> {
    fn parse(parser: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        Ok(Box::new(T::parse(parser)?))
    }
}
