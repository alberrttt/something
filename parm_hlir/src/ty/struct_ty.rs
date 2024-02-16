use std::collections::HashMap;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct StructTy<'a, 'b> {
    pub symbol: Symbol<'a, 'b>,
    pub fields: Vec<Symbol<'a, 'b>>,
}
