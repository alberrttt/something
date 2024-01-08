use super::prelude::*;
use ast::*;
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionIR<'a> {
    pub name: &'a str,
    pub code: Vec<IR>,
}

impl<'a> FunctionIR<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            code: Default::default(),
        }
    }
}
