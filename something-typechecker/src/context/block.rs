use std::{collections::HashMap, rc::Rc};

use something_frontend::{Ident};

use crate::{
    types::{sig::TypeSig},
};

pub use super::Context;

#[derive(Debug, Clone, Default)]
pub struct BlockContext {
    pub(crate) parent: Option<Rc<Context>>,
    pub(crate) variables: HashMap<Ident, TypeSig>,
}
impl BlockContext {
    pub fn get(&self, key: &Ident) -> Option<TypeSig> {
        match self.variables.get(key) {
            Some(tmp) => Some(tmp.clone()),
            None => {
                dbg!(&key);
                self.parent.as_ref().unwrap().get(key)
            }
        }
    }
    pub fn set(&mut self, key: Ident, value: TypeSig) -> Option<TypeSig> {
        self.variables.insert(key, value)
    }
}
impl BlockContext {
    // pub fn from_ast_block(
    //     mut ctx: BlockContext,
    //     value: &Block,
    //     evals_to: Primitive,
    // ) -> BlockContext {
    //     for node in value.iter() {
    //         dbg!(&ctx);

    //         match node {
    //             something_ast::Node::Statement(stmt) => match stmt {
    //                 something_frontend::Statement::Expression(expr, _) => {
    //                     expr.resolve(&mut ctx);
    //                 }
    //                 something_frontend::Statement::Return(_, expr, _) => {
    //                     let expr_type: TypeSig = expr.resolve(&mut ctx);
    //                     if expr_type == (&evals_to).into() {
    //                     } else {
    //                         panic!("Type mismatch")
    //                     }
    //                 }
    //             },
    //             something_ast::Node::Declaration(decl) => match decl {
    //                 something_frontend::Declaration::Function(_) => todo!(),
    //                 something_frontend::Declaration::Var(var_decl) => {
    //                     var_decl.resolve(&mut ctx);
    //                 }
    //             },
    //         };
    //     }
    //     ctx
    // }
}
