use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    // idx in the arena
    pub idx: usize,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ScopeRef<'a> {
    pub idx: usize,
    pub arena: *mut ScopeArena,
    pub _marker: std::marker::PhantomData<&'a ()>,
}
impl<'a> Deref for ScopeRef<'a> {
    type Target = Scope;
    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.arena).arena[self.idx] }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ScopeArena {
    arena: Vec<Scope>,
}

impl ScopeArena {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, scope: usize) {
        let arena_len = self.arena.len();
        let child = Scope {
            idx: arena_len,
            parent: Some(scope),
            children: vec![],
        };
        self.arena[scope].children.push(child.idx);
        self.arena.push(child);
    }

    pub fn push(&mut self) {
        let scope = Scope {
            idx: self.arena.len(),
            parent: None,
            children: vec![],
        };
        self.arena.push(scope);
    }
}
