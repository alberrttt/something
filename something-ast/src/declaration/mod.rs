use something_dev_tools::item_name;

use super::prelude::*;
mod function;
pub use self::function::*;
pub use self::var::*;

#[derive(Debug, ParseTokens, Clone, ParseTokensDisplay)]
pub enum Declaration {
    Function(FunctionDeclaration),
    Var(VariableDeclaration),
}
item_name!(Declaration, "declaration");
pub mod var;
