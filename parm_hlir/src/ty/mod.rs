pub mod function_ty;
pub mod struct_ty;
use crate::prelude::*;
use std::{fmt::Debug, marker::PhantomData, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub struct TypeArena<'a, 'b> {
    pub types: Vec<Type<'a, 'b>>,
}
#[derive(Clone, PartialEq)]
pub struct TypeRef<'a, 'b> {
    pub idx: usize,
    pub __: PhantomData<&'b &'a ()>,
}
impl<'a, 'b> Debug for TypeRef<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeRef").field("idx", &self.idx).finish()
    }
}
impl<'a, 'b> TypeRef<'a, 'b> {
    pub const fn new(idx: usize) -> Self {
        Self {
            idx,
            __: PhantomData,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Type<'a, 'b: 'a> {
    Number,
    StringLiteral,
    Boolean,
    Function(Rc<FunctionTy<'a, 'b>>),
    None(PhantomData<&'b &'a ()>),
}

impl<'a, 'b> Typechecker<'a, 'b> {
    pub fn default_types() -> Vec<Type<'a, 'b>> {
        vec![
            Type::None(PhantomData),
            Type::Number,
            Type::StringLiteral,
            Type::Boolean,
        ]
    }
    pub const NUMBER: TypeRef<'static, 'static> = TypeRef::new(1);
    pub const STRING_LITERAL: TypeRef<'static, 'static> = TypeRef::new(2);
    pub const BOOLEAN: TypeRef<'static, 'static> = TypeRef::new(3);
}
