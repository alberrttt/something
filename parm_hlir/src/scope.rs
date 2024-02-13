use std::rc::Rc;

use crate::symbol::Symbol;

#[derive(Debug, Clone, PartialEq)]
pub struct Scope<'a, 'b> {
    pub symbols: Vec<Symbol<'a, 'b>>,
    pub children: Vec<usize>,
    pub parent: Option<usize>,
    pub idx: usize,
}
impl<'a, 'b> Scope<'a, 'b> {
    pub fn get_symbol(&self, lex: &str) -> Option<Symbol<'a, 'b>> {
        for symbol in &self.symbols {
            let inner = symbol.inner.borrow();
            if inner.lexeme == lex {
                return Some(symbol.clone());
            }
        }
        None
    }

    pub fn push_symbol(&mut self, symbol: Symbol<'a, 'b>) {
        self.symbols.push(symbol);
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ScopeArena<'a, 'b> {
    pub scopes: Vec<Scope<'a, 'b>>,
}

impl<'a, 'b> ScopeArena<'a, 'b> {
    pub fn push(&mut self, parent: Option<usize>) -> usize {
        let idx = self.scopes.len();
        let scope = Scope {
            symbols: vec![],
            children: vec![],
            idx,
            parent,
        };
        self.scopes.push(scope);
        if let Some(parent) = parent {
            self.scopes[parent].children.push(idx);
        }
        idx
    }

    pub fn push_symbol(&mut self, symbol: Symbol<'a, 'b>, scope: usize) {
        self.scopes[scope].symbols.push(symbol);
    }

    pub fn get(&self, idx: usize) -> &Scope<'a, 'b> {
        &self.scopes[idx]
    }

    pub fn get_mut(&mut self, idx: usize) -> &mut Scope<'a, 'b> {
        &mut self.scopes[idx]
    }
}
