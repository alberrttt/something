use crate::types::Type;

pub trait ToType {
    fn to_type(&self) -> Type;
}