use crate::error::ParseError;

pub type ParseResult<'a, T> = Result<T, ParseError<'a>>;
