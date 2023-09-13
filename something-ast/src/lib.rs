#![feature(try_trait_v2)]
#![feature(concat_idents)]

pub mod ast;
pub mod error;
pub mod parser;
pub mod tokenizer;
pub use parser::Parser;
pub mod prelude {
    pub type ParseResult<T> = Result<T, ParseError>;
    pub use super::tokenizer::prelude::*;
    use crate::error::ParseError;
    pub use something_common::devprintln;
}
pub mod macros;
pub use prelude::ParseResult;
pub use tokenizer::prelude::*;
