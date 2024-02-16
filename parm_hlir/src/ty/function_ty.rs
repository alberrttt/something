use std::fmt::Debug;

use crate::symbol::Symbol;

use super::Type;

#[derive(Clone, PartialEq)]
pub struct FunctionTy<'a, 'b> {
    pub symbol: Symbol<'a, 'b>,
    pub return_ty: Type<'a, 'b>,
}
impl<'a, 'b> Debug for FunctionTy<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = &self.symbol.inner.borrow();
        f.debug_struct("FunctionTy")
            .field("symbol", &(&symbol.lexeme))
            .finish()
    }
}
