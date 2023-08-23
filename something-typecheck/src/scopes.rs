use std::{collections::HashSet, rc::Rc};

use something_ast::ast::{
    prelude::{Declaration, FunctionDeclaration},
    Node,
};

use crate::{
    symbol::{FnSig, Symbol},
    type_infer::InferLiteralType,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scope {
    pub symbols: Vec<Rc<Symbol>>,
}

impl Scope {
    pub fn create_scope_from_function(function: FunctionDeclaration, fn_sig: FnSig) -> Self {
        let mut symbols: Vec<_> = fn_sig.params.to_vec();
        for stmt in function.body.iter() {
            match stmt {
                Node::Declaration(decl) => match decl {
                    Declaration::Var(var) => {
                        
                        symbols.push(Rc::new(Symbol {
                            name: var.name.to_string(),
                            symbol_type: var.infer_literal_type().unwrap(),
                        }));
                    }
                    Declaration::Function(_) => todo!(),
                },
                Node::Statement(_) => todo!(),
            }
        }
        Self { symbols }
    }
}
