use std::{collections::HashMap, fs::File, rc::Rc};

use something_ast::{Ast, TopLevelNode};
use something_frontend::Ident;

use crate::{
    error::TypeError,
    types::sig::{FnSig, TypeSig},
};

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

impl TryFrom<Ast> for FileContext {
    type Error = TypeError;

    fn try_from(value: Ast) -> Result<Self, Self::Error> {
        let mut ctx = Self::default();
        for node in value.nodes.iter() {
            match node {
                TopLevelNode::FunctionDeclaration(fn_decl) => {
                    let fn_ctx = FnContext {
                        parent: Some(Rc::new(Context::File(ctx.clone()))),
                        ..Default::default()
                    }
                    .typecheck(fn_decl)?;
                    let fn_sig = FnSig::try_from(&fn_ctx)?;
                    ctx.variables
                        .insert(fn_decl.name.clone(), TypeSig::Fn(fn_sig));

                    ctx.fns.push(fn_ctx);
                }
            }
        }
        Ok(ctx)
    }
}
