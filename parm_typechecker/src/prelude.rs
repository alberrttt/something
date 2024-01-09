pub type TypeResult<'a, T> = Result<T, TypeError<'a>>;
pub use super::error::{
    display_diagnostic, ErrorKind, InvalidOperand, Mismatch, TypeError, UndefinedSymbol,
};
pub use super::TypeChecker;
pub use crate::types::Type;
pub use parm_ast::prelude as ast;
