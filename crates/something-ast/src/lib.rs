use error::ParseError;

pub mod ast;
pub mod error;
pub mod tokenizer;

pub type ParseResult<T> = something_common::Result<T, ParseError>;
