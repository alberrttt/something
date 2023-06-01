use std::{collections::HashMap, rc::Rc};

use something_frontend::{block::Block, Ident};

use crate::types::sig::TypeSig;

use self::{block::BlockContext, file::FileContext, function::FnContext};
#[derive(Debug, Clone)]
pub enum Context {
    Function(FnContext),
    File(FileContext),
    Block(BlockContext),
}
impl Context {
    pub fn set(&mut self, key: &Ident, value: TypeSig) {
        match self {
            Context::Function(fn_ctx) => fn_ctx.variables.insert(key.clone(), value),
            Context::File(file_ctx) => file_ctx.set(key.clone(), value),
            Context::Block(block_ctx) => block_ctx.set(key.clone(), value),
        };
    }
    pub fn get(&self, key: &Ident) -> Option<TypeSig> {
        match self {
            Context::Function(fn_ctx) => {
                //TODO: FOR SOME REASON, FN_CTX'S BLOCK IS EMPTY
                match fn_ctx.variables.get(key) {
                    Some(tmp) => Some(tmp.clone()),
                    None => match fn_ctx.parameters.get(key).cloned() {
                        None => fn_ctx.parent.as_ref()?.get(key),
                        Some(tmp) => Some(tmp),
                    },
                }
            }
            Context::File(file_ctx) => file_ctx.get(key),
            Context::Block(block_ctx) => block_ctx.get(key),
        }
    }
}
pub mod block;
pub mod file;
pub mod function;

impl BlockContext {
    fn new(block: Block) -> Self {
        let mut ctx = Self::default();
        for node in block.iter() {
            match node {
                something_ast::Node::Statement(stmt) => todo!(),
                something_ast::Node::Declaration(decl) => match decl {
                    something_frontend::Declaration::Function(_) => todo!(),
                    something_frontend::Declaration::Var(var_decl) => todo!(),
                },
            }
        }
        ctx
    }
}
