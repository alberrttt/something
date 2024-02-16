use std::collections::HashMap;

use crate::prelude::*;

#[derive(Clone, PartialEq)]
pub struct StructTy<'a, 'b> {
    pub symbol: Symbol<'a, 'b>,
    pub fields: Vec<Symbol<'a, 'b>>,
}
impl<'a, 'b> Debug for StructTy<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct Tmp<'t>(&'t str);
        impl<'t> Debug for Tmp<'t> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.0)
            }
        }
        f.debug_struct("StructTy")
            .field("symbol", &Tmp(self.symbol.inner.borrow().lexeme))
            .field("fields", &self.fields)
            .finish()
    }
}
