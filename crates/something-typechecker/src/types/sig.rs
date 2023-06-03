use std::fmt::Display;

use crate::{context::function::FnContext, error::TypeError};

use super::primitives::Primitive;
#[derive(Debug, Clone, PartialEq)]
pub struct FnSig(pub(crate) Vec<TypeSig>, pub(crate) Box<TypeSig>);
impl TryFrom<&FnContext> for FnSig {
    type Error = TypeError;

    fn try_from(value: &FnContext) -> Result<Self, Self::Error> {
        let mut params = Vec::new();
        for (_, ty) in value.parameters.iter() {
            params.push(ty.clone());
        }
        let ret = TypeSig::Primitive(Primitive::Void);
        Ok(FnSig(params, Box::new(ret)))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum TypeSig {
    Primitive(Primitive),
    Fn(FnSig),
}
impl Display for TypeSig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeSig::Primitive(p) => write!(f, "{}", p),
            TypeSig::Fn(FnSig(params, ret)) => {
                write!(f, "fn(")?;
                for param in params {
                    write!(f, "{}, ", param)?;
                }
                write!(f, ") -> {}", ret)
            }
        }
    }
}
impl From<&Primitive> for TypeSig {
    fn from(value: &Primitive) -> Self {
        TypeSig::Primitive(value.clone())
    }
}
impl From<Primitive> for TypeSig {
    fn from(value: Primitive) -> Self {
        TypeSig::Primitive(value)
    }
}
