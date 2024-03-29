use super::TokenStream;
use crate::prelude::*;
use something_dev_tools::tuple_parse_impl;

pub trait AppendTokens {
    fn append_tokens(&self, tokens: &mut TokenStream);
}
impl<T> AppendTokens for Box<T>
where
    T: AppendTokens,
{
    fn append_tokens(&self, tokens: &mut TokenStream)
    where
        Self: Sized,
    {
        (**self).append_tokens(tokens)
    }
}
pub trait ToTokens {
    fn to_tokens(&self) -> TokenStream;
}
impl<T: AppendTokens> ToTokens for T {
    fn to_tokens(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.append_tokens(&mut tokens);
        tokens
    }
}
pub trait Parse: ParsingDisplay + std::fmt::Debug {
    fn parse(parser: &mut crate::parser::Parser) -> ParseResult<Self>
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
    fn parse(_parser: &mut crate::parser::Parser) -> ParseResult<Self> {
        Ok(())
    }
}

tuple_parse_impl!(A, B, C, D, E, F);
tuple_parse_impl!(A, B, C, D, E);
tuple_parse_impl!(A, B, C, D);
tuple_parse_impl!(A, B, C);
tuple_parse_impl!(A, B);
use crate::ast;
use crate::ast::prelude::*;
use crate::tokenizer::prelude::*;
#[test]
fn test_tuple() {
    let mut tokens = crate::parser::Parser::new("", "a b c d e f");
    type idents = (Ident, Ident, Ident, Ident, Ident, Ident);
    let idents: idents = Parse::parse(&mut tokens).unwrap();
    assert_eq!(idents.0.to_string(), "a");
    assert_eq!(idents.1.to_string(), "b");
    assert_eq!(idents.2.to_string(), "c");
    assert_eq!(idents.3.to_string(), "d");
    assert_eq!(idents.4.to_string(), "e");
    assert_eq!(idents.5.to_string(), "f");
}
