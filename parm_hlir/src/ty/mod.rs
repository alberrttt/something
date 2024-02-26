pub mod function_ty;
pub mod struct_ty;
use parm_ast::parser::nodes::type_nodes::TypeExpression;
use parm_common::Spanned;

use crate::{error::TypeError, prelude::*};
use std::{fmt::Debug, fmt::Display, marker::PhantomData, process::Output, rc::Rc};

use self::{struct_ty::StructTy, traits::TypeCheckResult};

#[derive(Debug, Clone, PartialEq)]
pub struct TypeArena<'a, 'b> {
    pub types: Vec<Type<'a, 'b>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type<'a, 'b> {
    Uint(UintTy),
    Int(IntTy),
    Float(FloatTy),
    StringLiteral,
    Boolean,
    Struct(Rc<StructTy<'a, 'b>>),
    Function(Rc<FunctionTy<'a, 'b>>),
    Unknown { err: bool },
    Any,
}
impl<'a, 'b> Display for Type<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Uint(ty) => write!(f, "{}", ty),
            Type::Int(ty) => write!(f, "{}", ty),
            Type::Float(ty) => write!(f, "{}", ty),
            Type::StringLiteral => write!(f, "stringlit"),
            Type::Boolean => write!(f, "boolean"),
            Type::Struct(ty) => write!(f, "{}", ty.symbol.inner.borrow().lexeme),
            Type::Function(ty) => write!(f, "{}", ty.symbol.inner.borrow().lexeme),
            Type::Unknown { err } => write!(f, "Unknown"),
            Type::Any => write!(f, "Any"),
        }
    }
}
impl<'a, 'b> Type<'a, 'b> {
    pub fn function(ty: FunctionTy<'a, 'b>) -> Self {
        Type::Function(Rc::new(ty))
    }

    pub fn is_ambigious_int(&self) -> bool {
        matches!(
            self,
            Type::Int(IntTy::Ambiguous) | Type::Uint(UintTy::Ambiguous)
        )
    }
    pub fn is_ambigious_uint(&self) -> bool {
        matches!(self, Type::Uint(UintTy::Ambiguous))
    }

    /// eq, accounting for ambigious types
    pub fn eq_amb(&self, other: &Self) -> bool {
        self == other
            || (self.is_ambigious_int() && matches!(other, Type::Int(_)))
            || (self.is_ambigious_uint() && matches!(other, Type::Uint(_)))
            || (other.is_ambigious_uint() && matches!(self, Type::Uint(_)))
            || (other.is_ambigious_int() && matches!(self, Type::Int(_)))
            // if any are unknown, we will assume the programmer intended for the correct type.
            || matches!(self, Type::Unknown { err })
            || matches!(other, Type::Unknown { err})
            || matches!(self, Type::Any)
            || matches!(other, Type::Any)
    }
}

#[derive(Debug, Clone)]
pub enum UintTy {
    U8,
    U16,
    U32,
    U64,
    U128,
    Ambiguous,
}
impl std::fmt::Display for UintTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UintTy::U8 => write!(f, "u8"),
            UintTy::U16 => write!(f, "u16"),
            UintTy::U32 => write!(f, "u32"),
            UintTy::U64 => write!(f, "u64"),
            UintTy::U128 => write!(f, "u128"),
            UintTy::Ambiguous => write!(f, "{{natural number}}"),
        }
    }
}
impl PartialEq for UintTy {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
            || matches!(
                (self, other),
                (UintTy::Ambiguous, _) | (_, UintTy::Ambiguous)
            )
    }
}
#[derive(Debug, Clone)]
pub enum IntTy {
    I8,
    I16,
    I32,
    I64,
    I128,
    Ambiguous,
}
impl std::fmt::Display for IntTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntTy::I8 => write!(f, "i8"),
            IntTy::I16 => write!(f, "i16"),
            IntTy::I32 => write!(f, "i32"),
            IntTy::I64 => write!(f, "i64"),
            IntTy::I128 => write!(f, "i128"),
            IntTy::Ambiguous => write!(f, "{{integer}}"),
        }
    }
}
impl PartialEq for IntTy {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
            || matches!((self, other), (IntTy::Ambiguous, _) | (_, IntTy::Ambiguous))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FloatTy {
    F32,
    F64,
}
impl Display for FloatTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FloatTy::F32 => write!(f, "f32"),
            FloatTy::F64 => write!(f, "f64"),
        }
    }
}
impl<'a, 'b> Check<'a, 'b, Type<'a, 'b>> for TypeExpression<'a> {
    fn check(&self, tc: &mut Typechecker<'a, 'b>) -> TypeCheckResult<'a, 'b, Type<'a, 'b>> {
        let segment = self.path.segments.elements();
        let first = segment.first().unwrap();
        assert_eq!(segment.len(), 1); // for now

        let ty = first.ident.lexeme;

        Ok(match ty {
            "u8" => Type::Uint(UintTy::U8),
            "u16" => Type::Uint(UintTy::U16),
            "u32" => Type::Uint(UintTy::U32),
            "u64" => Type::Uint(UintTy::U64),
            "u128" => Type::Uint(UintTy::U128),
            "i8" => Type::Int(IntTy::I8),
            "i16" => Type::Int(IntTy::I16),
            "i32" => Type::Int(IntTy::I32),
            "i64" => Type::Int(IntTy::I64),
            "i128" => Type::Int(IntTy::I128),
            "f32" => Type::Float(FloatTy::F32),
            "f64" => Type::Float(FloatTy::F64),
            "bool" => Type::Boolean,

            ty => {
                return Err(TypeError::new(
                    error::TypeErrorKind::TypeNameNotFound {
                        name: ty,
                        location: self.span(),
                    },
                    tc.source_file.preparsed,
                ))
            }
        })
    }
}
