use std::collections::HashMap;

use crate::prelude::*;
#[derive(Default, Debug, Clone)]
pub struct TypeChecker<'a> {
    pub ast: Ast,
    pub globals: HashMap<&'a Ident, FunctionType>,
}
