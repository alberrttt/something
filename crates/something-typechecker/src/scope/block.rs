use std::collections::HashMap;

use something_frontend::Ident;

use crate::prelude::Type;
#[derive(Debug, Clone)]
pub struct BlockScope {
    pub variables: HashMap<Ident, Type>,
}
impl BlockScope {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
    pub fn set(&mut self, name: &Ident, ty: Type) {
        self.variables.insert(name.clone(), ty);
    }
}
