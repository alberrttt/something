use super::nodes::{
    declaration::{function::Function, variable::Variable},
    statement::Statement,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Item<'a> {
    Variable(Variable<'a>),
    Function(Function<'a>),
}
