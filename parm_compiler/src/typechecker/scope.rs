use std::{
    cell::{RefCell, UnsafeCell},
    collections::HashMap,
    default,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use super::{symbol::Symbol, ty::TypeRef};

#[derive(Debug)]
pub struct Scope<'a> {
    // idx in the arena
    pub idx: usize,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub vars: HashMap<&'a str, Symbol<'a>>,
}
impl<'a> Scope<'a> {
    pub fn new(idx: usize, parent: Option<usize>) -> Self {
        Self {
            idx,
            parent,
            children: vec![],
            vars: HashMap::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ScopeArena<'a> {
    pub arena: Vec<Rc<RefCell<Scope<'a>>>>,
}

impl<'a> ScopeArena<'a> {
    pub fn new() -> Self {
        Self::default()
    }
    /// scope: the parent of the new scope
    pub fn insert(&'a mut self, scope: usize) -> Rc<RefCell<Scope<'a>>> {
        let arena_len = self.arena.len();
        let child = Scope::new(arena_len, Some(scope));
        {
            let mut scope = &self.arena[scope];
            let mut scope = scope.borrow_mut();
            scope.children.push(child.idx);
        }
        let reference = Rc::new(RefCell::new(child));
        self.arena.push(reference.clone());
        reference
    }

    pub fn push(&mut self) -> Rc<RefCell<Scope<'a>>> {
        let scope = Scope::new(self.arena.len(), None);
        let reference = Rc::new(RefCell::new(scope));
        self.arena.push(reference.clone());
        reference
    }

    pub fn get_symbol(&self, from: usize, name: &str) -> Option<Symbol<'a>> {
        let scope = &self.arena[from];
        let scope = scope.borrow();
        for (variable, symbol) in &scope.vars {
            if variable.eq(&name) {
                return Some(symbol.clone());
            }
        }
        if let Some(parent) = scope.parent {
            self.get_symbol(parent, name)
        } else {
            None
        }
    }
    pub fn get_ty(&self, from: usize, name: &str) -> Option<TypeRef<'a>> {
        self.get_symbol(from, name)
            .map(|symbol| symbol.inner.borrow().ty.clone())
    }
}