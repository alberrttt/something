use something_dev_tools::Node;

use crate::prelude::*;

use super::prelude::{block::Block, Declaration, Expression};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Declaration(Declaration),
    Expression(Expression, Option<Semicolon>),
}
