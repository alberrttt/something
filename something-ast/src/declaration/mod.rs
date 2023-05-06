use super::prelude::*;
mod function;
pub use self::function::*;
pub use self::var::*;

#[derive(Debug, ParseTokens, Clone, ParseTokensDisplay)]
pub enum Declaration {
    Function(FunctionDeclaration),
    Var(VariableDeclaration),
}

pub mod var;
