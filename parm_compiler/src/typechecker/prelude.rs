pub type TypeResult<'a, T> = Result<T, TypeError<'a>>;
pub use super::error::{
    display_diagnostic, ErrorKind, InvalidOperand, Mismatch, TypeError, UndefinedSymbol,
};
pub use super::types::{FnSig, FunctionSig, Type};
pub use super::TypeChecker;
pub use crate::ast::prelude as ast;
