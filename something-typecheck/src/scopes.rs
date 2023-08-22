use std::collections::HashSet;

use something_ast::ast::{
    prelude::{Declaration, FunctionDeclaration},
    Node,
};

use crate::{
    symbol::{FnType, Symbol},
    type_infer::InferLiteralType,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scope {
    pub symbols: HashSet<Symbol>,
}

impl Scope {
    pub fn create_scope_from_function(function: FunctionDeclaration, fn_sig: FnType) -> Self {
        let mut symbols = HashSet::from_iter(fn_sig.params.clone());
        for stmt in function.body.iter() {
            match stmt {
                Node::Declaration(decl) => match decl {
                    Declaration::Var(var) => {
                        symbols.insert(Symbol {
                            name: var.name.to_string(),
                            symbol_type: var.infer_literal_type().unwrap(),
                        });
                    }
                    Declaration::Function(_) => todo!(),
                },
                Node::Statement(_) => todo!(),
            }
        }
        Self { symbols }
    }
}
