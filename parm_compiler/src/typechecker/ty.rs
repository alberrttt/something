#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub data: TypeData,
}
#[derive(Debug, Clone, PartialEq)]
pub enum TypeData {
    Number,
    String,
    Boolean,
}

impl Type {
    pub fn ty_expr<'a>(ty: &TypeExpression<'a>) -> Self {
        let name = ty.path.segments.last.as_ref().unwrap().ident.lexeme;
        match name {
            "number" => Self {
                data: TypeData::Number,
            },
            "string" => Self {
                data: TypeData::String,
            },
            "bool" => Self {
                data: TypeData::Boolean,
            },
            _ => panic!(),
        }
    }
}
impl Type {
    pub fn allocate(self, arena: &mut TypeArena) -> TypeRef {
        // first, lets see if we already have this type
        for (idx, ty) in arena.types.iter().enumerate() {
            if ty == &self {
                return TypeRef {
                    idx,
                    arena: arena as *mut TypeArena,
                    _marker: std::marker::PhantomData,
                };
            }
        }
        let idx = arena.types.len();

        arena.types.push(self);
        TypeRef {
            idx,
            arena: arena as *mut TypeArena,
            _marker: std::marker::PhantomData,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TypeArena {
    pub types: Vec<Type>,
}

#[derive(Clone, PartialEq)]
pub struct TypeRef<'a> {
    pub idx: usize,
    pub arena: *mut TypeArena,
    pub _marker: std::marker::PhantomData<&'a ()>,
}
// Recursive expansion of Debug macro
// ===================================

impl std::fmt::Debug for TypeRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let TypeRef {
            idx,
            arena,
            _marker,
        } = self;
        f.debug_struct("TypeRef")
            .field("idx", &idx)
            .field("arena", &arena)
            .field("data", &**self)
            .finish()
    }
}
use std::{
    fmt::{self, Debug},
    ops::*,
};

use crate::ast::prelude::TypeExpression;
impl<'a> Deref for TypeRef<'a> {
    type Target = Type;
    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.arena).types[self.idx] }
    }
}
