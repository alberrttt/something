#![feature(pointer_is_aligned)]
#![feature(lazy_cell)]
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};
pub mod error;
pub mod expression;
pub mod item;
pub mod path;
pub mod prelude;
pub mod scope;
pub mod statement;
pub mod symbol;
pub mod traits;
pub mod ty;
pub mod typechecker;
#[derive(PartialEq, Clone)]
pub struct AST<T>(T);
impl<T> Debug for AST<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("AST<{}>", std::any::type_name::<T>());
        f.debug_tuple(&name).finish()
    }
}

impl<T> Deref for AST<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for AST<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
fn x() {}
