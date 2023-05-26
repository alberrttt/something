use std::{collections::HashMap, rc::Rc};

use crate::prelude::*;
use something_frontend::{block::Block, Ident};
#[derive(Debug, Clone, Default)]
pub struct BlockCtx {
    pub vars: HashMap<Ident, Type>,
    pub parent: Option<Rc<BlockCtx>>,
}
impl BlockCtx {
    pub fn set(&mut self, name: Ident, ty: Type) {
        self.vars.insert(name, ty);
    }
    pub fn get_var(&self, name: &Ident) -> Option<&Type> {
        self.vars.get(name)
    }
    pub fn get_var_recursive(&self, name: &Ident) -> Option<&Type> {
        if let Some(ty) = self.get_var(&name) {
            Some(ty)
        } else if let Some(parent) = &self.parent {
            parent.get_var_recursive(name)
        } else {
            None
        }
    }
}
