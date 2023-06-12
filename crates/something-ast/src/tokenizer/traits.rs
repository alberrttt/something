use std::{error::Error, fmt::Debug};

use super::{error::ParseError, ident::Ident, Tokens};
use crate::prelude::*;
use something_dev_tools::tuple_parse_impl;

pub trait AppendTokens {
    fn append_tokens(&self, tokens: &mut Tokens)
    where
        Self: Sized;
}
impl<T> AppendTokens for Box<T>
where
    T: AppendTokens,
{
    fn append_tokens(&self, tokens: &mut Tokens)
    where
        Self: Sized,
    {
        (**self).append_tokens(tokens)
    }
}
pub trait Parse: ParsingDisplay + std::fmt::Debug {
    fn parse(input: &mut Tokens) -> ParseResult<Self>
    where
        Self: Sized;
}
pub trait ParsingDisplay {
    fn display(&self) -> String
    where
        Self: Sized;
    fn placeholder() -> String
    where
        Self: Sized;
}
impl<T> ParsingDisplay for Box<T>
where
    T: ParsingDisplay,
{
    fn display(&self) -> String
    where
        Self: Sized,
    {
        (**self).display()
    }
    fn placeholder() -> String
    where
        Self: Sized,
    {
        T::placeholder()
    }
}
impl Parse for () {
    fn parse(_input: &mut Tokens) -> ParseResult<Self> {
        Ok(())
    }
}

tuple_parse_impl!(A, B, C, D, E, F);
tuple_parse_impl!(A, B, C, D, E);
tuple_parse_impl!(A, B, C, D);
tuple_parse_impl!(A, B, C);
tuple_parse_impl!(A, B);

#[test]
fn test_tuple() {
    let mut tokens = Tokens::from("a b c d e f");
    type idents = (Ident, Ident, Ident, Ident, Ident, Ident);
    let idents: idents = Parse::parse(&mut tokens).unwrap();
    assert_eq!(idents.0.to_string(), "a");
    assert_eq!(idents.1.to_string(), "b");
    assert_eq!(idents.2.to_string(), "c");
    assert_eq!(idents.3.to_string(), "d");
    assert_eq!(idents.4.to_string(), "e");
    assert_eq!(idents.5.to_string(), "f");
}
