use std::collections::HashMap;

use something_frontend::Ident;

use crate::{prelude::Type, traits::Scope};
#[derive(Debug, Clone)]
pub struct BlockScope {
    pub variables: HashMap<Ident, Type>,
}
impl Scope for BlockScope {
    fn get(&self, name: &Ident) -> Option<Type> {
        self.variables.get(name).cloned()
    }

    fn set(&mut self, name: &Ident, ty: Type) {
        self.variables.insert(name.clone(), ty);
    }
}
impl BlockScope {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}
