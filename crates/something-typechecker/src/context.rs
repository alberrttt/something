use std::{collections::HashMap, rc::Rc};

use crate::prelude::*;
use something_frontend::Ident;
#[derive(Debug, Clone, Default)]
pub struct BlockScope {
    pub vars: HashMap<Ident, Type>,
    pub parent: Option<Rc<BlockScope>>,
    pub modules: Vec<Rc<Module>>,
}
#[derive(Debug, Clone)]
pub struct FnCtx {
    pub block: BlockScope,
    pub name: Ident,
}
impl TryFrom<FunctionDeclaration> for FnCtx {
    type Error = TypeError;

    fn try_from(value: FunctionDeclaration) -> Result<Self, Self::Error> {
        Ok(Self {
            block: (&value).try_into()?,
            name: value.name,
        })
    }
}
impl FnCtx {
    pub fn new(name: Ident) -> Self {
        Self {
            block: BlockScope::default(),
            name,
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct FileScope {
    pub fns: Vec<FnCtx>,
}
impl BlockScope {
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

fn __test() {
    let mut ctx = BlockScope::default();
    ctx.set("a".into(), Type::number());
    ctx.set("b".into(), Type::number());
    let mut ctx2 = BlockScope {
        parent: Some(Rc::new(ctx)),
        ..BlockScope::default()
    };
    ctx2.set("c".into(), Type::boolean());
    dbg!(ctx2.get_var_recursive(&"a".into()));
    dbg!(ctx2.get_var_recursive(&"b".into()));
    dbg!(ctx2.get_var_recursive(&"c".into()));
}

#[test]
fn test() {
    __test()
}
