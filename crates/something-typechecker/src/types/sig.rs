use std::fmt::Display;

use super::primitives::Primitive;
pub type FnSig = (Vec<TypeSig>, Box<TypeSig>);
#[derive(Debug, Clone, PartialEq)]
pub enum TypeSig {
    Primitive(Primitive),
    Fn(FnSig),
}
impl Display for TypeSig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeSig::Primitive(p) => write!(f, "{}", p),
            TypeSig::Fn((params, ret)) => {
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
