use std::{collections::HashMap, fmt::Debug, rc::Rc};

use crate::{prelude::Type, symbol::Symbol};

pub trait Scope {
    fn get(&self, symbol: Rc<Symbol>) -> Option<Type>;
    fn set(&mut self, symbol: Symbol, ty: Type);
}

#[derive(Default, Clone)]
pub struct BlockCtx {
    pub parent: Option<Rc<dyn Scope>>,
    pub symbols: HashMap<Rc<Symbol>, Type>,
    pub should_eval_to: Type,
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
