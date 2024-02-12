use crate::symbol::Symbol;

use super::TypeRef;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionTy<'a, 'b> {
    pub symbol: Symbol<'a, 'b>,
    pub return_ty: TypeRef<'a, 'b>,
}
