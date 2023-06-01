use std::{collections::HashMap, fs::File, rc::Rc};

use something_ast::Ast;
use something_frontend::Ident;

use crate::types::sig::{FnSig, TypeSig};

use super::{Context, FnContext};

#[derive(Debug, Clone, Default)]
pub struct FileContext {
    variables: HashMap<Ident, TypeSig>,
    fns: Vec<FnContext>,
}
impl FileContext {
    pub fn get(&self, key: &Ident) -> Option<TypeSig> {
        self.variables.get(key).cloned()
    }
    pub fn set(&mut self, key: Ident, value: TypeSig) -> Option<TypeSig> {
        self.variables.insert(key, value)
    }
}
impl From<Ast> for FileContext {
    fn from(value: Ast) -> Self {
        let mut ctx = Self::default();
        for node in value.nodes.iter() {
            match node {
                something_ast::TopLevelNode::FunctionDeclaration(fn_decl) => {
                    let mut fn_ctx = FnContext::from_fn_decl(
                        FnContext {
                            parent: Some(Rc::new(Context::File(ctx.clone()))),
                            ..Default::default()
                        },
                        fn_decl,
                    );
                    let fn_sig: FnSig = FnSig::from(&fn_ctx);
                    ctx.variables
                        .insert(fn_decl.name.clone(), TypeSig::Fn(fn_sig));
                    ctx.fns.push(fn_ctx);
                }
            }
        }
        ctx
    }
}
