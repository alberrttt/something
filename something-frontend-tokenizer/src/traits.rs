use std::{error::Error, fmt::Display};

use crate::Tokens;

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
