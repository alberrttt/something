use error::ParseError;

pub mod ast;
pub mod error;
pub mod tokenizer;

pub mod prelude {
    pub type ParseResult<T> = something_common::Result<T, ParseError>;
    pub use something_common::Result::*;

    use crate::error::ParseError;
}
