use std::{
    cell::UnsafeCell,
    collections::HashMap,
    default,
    ops::{Deref, DerefMut},
};

use super::symbol::Symbol;

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

#[derive(Debug, Clone, PartialEq)]
pub struct MutScopeRef<'a> {
    pub idx: usize,
    pub arena: *mut ScopeArena<'a>,
    pub _marker: std::marker::PhantomData<&'a ()>,
}
impl<'a> MutScopeRef<'a> {
    pub fn new(idx: usize, arena: *mut ScopeArena<'a>) -> Self {
        Self {
            idx,
            arena,
            _marker: std::marker::PhantomData,
        }
    }
}
impl<'a> Deref for MutScopeRef<'a> {
    type Target = Scope<'a>;
    fn deref(&self) -> &Self::Target {
        let scope = unsafe { &(*self.arena).arena[self.idx] };

        scope
    }
}
impl<'a> DerefMut for MutScopeRef<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let scope = unsafe { &mut (*self.arena).arena[self.idx] };

        scope
    }
}

#[derive(Debug, Default)]
pub struct ScopeArena<'a> {
    arena: Vec<Scope<'a>>,
}

impl<'a> ScopeArena<'a> {
    pub fn new() -> Self {
        Self::default()
    }
    /// scope: the parent of the new scope
    pub fn insert(&mut self, scope: usize) -> MutScopeRef<'a> {
        let arena_len = self.arena.len();
        let child = Scope::new(arena_len, Some(scope));
        self.arena[scope].children.push(child.idx);
        self.arena.push(child);
        MutScopeRef::new(arena_len, self as *mut Self)
    }

    pub fn push(&mut self) -> MutScopeRef<'a> {
        let scope = Scope::new(self.arena.len(), None);
        self.arena.push(scope);
        MutScopeRef::new(self.arena.len() - 1, self as *mut Self)
    }
}
