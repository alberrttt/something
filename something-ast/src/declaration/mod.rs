use something_dev_tools::ParseTokens;
use something_frontend_tokenizer::tokens::Parse;
use something_frontend_tokenizer::Tokens;
use something_frontend_tokenizer::{
    tokens::{Let, Token},
    Token,
};
mod function;
pub use self::function::*;
pub use self::var::*;

#[derive(Debug, ParseTokens, Clone)]
pub enum Declaration {
    Function(FunctionDeclaration),
    Var(VariableDeclaration),
}

pub mod var;
