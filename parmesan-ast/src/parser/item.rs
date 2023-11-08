use crate::traits::Node;

use super::nodes::{
    declaration::{function::Function, variable::Variable},
    statement::Statement,
};
use parmesan_common::Spanned;
use parmesan_dev_macros::Spanned;

#[derive(Debug, Clone, PartialEq, Spanned)]
pub enum Item<'a> {
    Variable(Variable<'a>),
    Function(Function<'a>),
}
impl<'a> Node<'a> for Item<'a> {
    fn parse<'b: 'a>(
        parser: &'b mut super::Parser<'a>,
    ) -> Result<Self, crate::error::ParseError<'a>>
    where
        Self: Sized,
    {
        todo!()
    }
}
