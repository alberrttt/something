#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    pub idx: usize,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScopeArena {
    arena: Vec<Scope>,
}

impl ScopeArena {
    pub fn new() -> Self {
        Self { arena: Vec::new() }
    }
}
