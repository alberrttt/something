use std::fmt::Debug;

use crate::symbol::Symbol;

use super::TypeRef;

#[derive(Clone, PartialEq)]
pub struct FunctionTy<'a, 'b> {
    pub symbol: Symbol<'a, 'b>,
    pub return_ty: TypeRef<'a, 'b>,
}
impl<'a, 'b> Debug for FunctionTy<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = &self.symbol.inner.borrow();
        let tc = unsafe { &*symbol.tc };
        f.debug_struct("FunctionTy")
            .field("symbol", &(&symbol.lexeme, &symbol.id))
            .field("return_ty", &tc.types_arena.types[self.return_ty.idx])
            .finish()
    }
}
