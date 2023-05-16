use something_dev_tools::item_name;

use super::prelude::*;
mod function;
pub use self::function::*;
pub use self::var::*;

#[derive(ParseTokens, Clone, ParseTokensDisplay)]
pub enum Declaration {
    Function(FunctionDeclaration),
    Var(VariableDeclaration),
}
impl std::fmt::Debug for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Function(arg0) => write!(f, "{:#?}", arg0),
            Self::Var(arg0) => write!(f, "{:#?}", arg0),
        }
    }
}
item_name!(Declaration, "declaration");
pub mod var;
