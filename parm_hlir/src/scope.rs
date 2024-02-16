use std::{collections::HashMap, rc::Rc};

use crate::symbol::Symbol;

#[derive(Debug, Clone, PartialEq)]
pub struct Scope<'a, 'b> {
    pub symbols: HashMap<&'a str, Symbol<'a, 'b>>,
    pub children: Vec<usize>,
    pub parent: Option<usize>,
    pub idx: usize,
}
impl<'a, 'b> Scope<'a, 'b> {
    pub fn get_symbol(&self, lex: &str) -> Option<Symbol<'a, 'b>> {
        for (idx, symbol) in &self.symbols {
            let inner = symbol.inner.borrow();
            if inner.lexeme == lex {
                return Some(symbol.clone());
            }
        }
        None
    }

    pub fn push_symbol(&mut self, symbol: Symbol<'a, 'b>) {
        let key = symbol.inner.borrow_mut().lexeme;
        self.symbols.insert(key, symbol);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScopeArena<'a, 'b> {
    pub scopes: Vec<Scope<'a, 'b>>,
}

impl<'a, 'b> ScopeArena<'a, 'b> {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope {
                symbols: HashMap::new(),
                children: vec![],
                idx: 0,
                parent: None,
            }],
        }
    }
}
impl<'a, 'b> ScopeArena<'a, 'b> {
    pub fn push(&mut self, parent: Option<usize>) -> usize {
        let idx = self.scopes.len();
        let scope = Scope {
            symbols: HashMap::new(),
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
        let key = symbol.inner.borrow().lexeme;
        self.scopes[scope].symbols.insert(key, symbol);
    }

    pub fn get(&self, idx: usize) -> &Scope<'a, 'b> {
        &self.scopes[idx]
    }

    pub fn get_mut(&mut self, idx: usize) -> &mut Scope<'a, 'b> {
        &mut self.scopes[idx]
    }
}
