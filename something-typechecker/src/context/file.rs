use std::{collections::HashMap, rc::Rc};

use something_ast::ast::{Ast, TopLevelNode};
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
    pub fn typecheck(self, ast: Ast) -> Result<Self, TypeError> {
        let mut ctx = self;
        for node in ast.nodes.iter() {
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

impl TryFrom<Ast> for FileContext {
    type Error = TypeError;

    fn try_from(ast: Ast) -> Result<Self, Self::Error> {
        let ctx = Self::default();
        ctx.typecheck(ast)
    }
}
