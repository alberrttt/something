use std::{collections::HashMap, fmt::Debug, rc::Rc, slice::Iter};

use crate::{prelude::Type, symbol::Symbol};
pub enum ScopeTypes {
    Block(BlockCtx),
}
pub trait Scope {
    fn get(&self, symbol: Rc<Symbol>) -> Option<Type>;
    fn set(&mut self, symbol: Symbol, ty: Type);
}

#[derive(Default, Clone)]
pub struct BlockCtx {
    pub parent: Option<Rc<ScopeTypes>>,
    pub symbols: HashMap<Rc<Symbol>, Type>,
    pub should_eval_to: Type,
}
impl BlockCtx {
    pub fn parents(&self) -> std::vec::IntoIter<std::rc::Rc<ScopeTypes>> {
        let mut parents = vec![];
        let mut current = self.parent.clone();
        while let Some(parent) = current {
            parents.push(parent.clone());
            current = match parent.as_ref() {
                ScopeTypes::Block(block) => block.parent.clone(),
            }
        }
        parents.into_iter()
    }
}
impl Scope for BlockCtx {
    fn get(&self, symbol: Rc<Symbol>) -> Option<Type> {
        self.symbols.get(&symbol).cloned()
    }

    fn set(&mut self, symbol: Symbol, ty: Type) {
        self.symbols.insert(Rc::new(symbol), ty);
    }
}
impl Debug for BlockCtx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockCtx")
            .field("symbols", &self.symbols)
            .field("should_eval_to", &self.should_eval_to)
            .finish()
    }
}
