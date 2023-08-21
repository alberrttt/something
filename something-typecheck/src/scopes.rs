use std::collections::HashSet;

use something_ast::ast::prelude::FunctionDeclaration;

use crate::symbol::Symbol;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scope {
    pub symbols: HashSet<Symbol>,
}

impl Scope {
    fn create_scope_from_function(function: &FunctionDeclaration) -> Self {
        let mut symbols = HashSet::new();

        Self { symbols }
    }
}
