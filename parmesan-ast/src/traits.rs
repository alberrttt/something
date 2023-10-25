use parmesan_common::Spanned;

use crate::{
    error::ParseError,
    parser::{nodes::Node, Parser},
};

pub trait Parse: Spanned {
    fn parse<'src>(parser: &mut Parser<'src>) -> Result<Self, Box<dyn ParseError>>
    where
        Self: Sized;
}
