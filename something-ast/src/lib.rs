#![feature(try_trait_v2)]
#![feature(concat_idents)]
#![feature(associated_type_defaults)]
pub mod ast;
pub mod error;
pub mod parser;
pub mod tokenizer;
pub use parser::Parser;
pub mod prelude {
    pub type ParseResult<T> = Result<T, ParseError>;
    pub use super::ast::nodes::*;
    pub use super::parser::*;
    pub use super::tokenizer::prelude::*;
    pub use crate::error::ParseError;
    pub use crate::{node};
    pub use something_common::devprintln;
}
pub mod macros;
pub use prelude::ParseResult;
pub use tokenizer::prelude::*;
