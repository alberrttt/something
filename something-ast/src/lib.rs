#![feature(try_trait_v2)]
#![feature(concat_idents)]

pub mod ast;
pub mod error;
pub mod tokenizer;

pub mod prelude {
    pub type ParseResult<T> = something_common::Result<T, ParseError>;
    pub use something_common::Result::*;

    use crate::error::ParseError;
}
pub mod macros;