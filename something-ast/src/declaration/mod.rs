use something_dev_tools::ParseTokens;
use something_frontend_tokenizer::{
    tokens::{Let, Parse, Token},
    Token,
};
mod function;
pub use self::function::*;
pub use self::var::*;
#[derive(Debug, ParseTokens)]
pub enum Declaration {
    Var(VariableDeclaration),
    Function(FunctionDeclaration),
}

pub mod var;
