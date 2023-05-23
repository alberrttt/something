use std::{
    error::Error,
    fmt::{Debug, Display},
};

use something_dev_tools::tuple_parse_impl;

use crate::{ident::Ident, Tokens};

pub trait Parse: ParsingDisplay + std::fmt::Debug {
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn Error>>
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
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn Error>> {
        Ok(())
    }
}

tuple_parse_impl!(A, B, C, D, E, F);
tuple_parse_impl!(A, B, C, D, E);
tuple_parse_impl!(A, B, C, D);
tuple_parse_impl!(A, B, C);
tuple_parse_impl!(A, B);

impl<T> Parse for Option<T>
where
    T: Debug + Parse + Clone,
{
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        let tmp = match input.step(|f| match T::parse(f) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Ok(None),
        }) {
            Ok(ok) => Ok(ok),
            Err(err) => Ok(None),
        };
        dbg!(&tmp);
        tmp
    }
}
impl<T> ParsingDisplay for Option<T> {
    fn display(&self) -> String
    where
        Self: Sized,
    {
        todo!()
    }

    fn placeholder() -> String
    where
        Self: Sized,
    {
        todo!()
    }
}

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
