use std::{marker::PhantomData, rc::Rc};

use crate::function::Function;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeArena<'a, 'b> {
    pub types: Vec<Type<'a, 'b>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type<'a, 'b: 'a> {
    Number,
    StringLiteral,
    Boolean,
    Function(Rc<Function<'a, 'b>>),
    __(PhantomData<&'b &'a ()>),
}
