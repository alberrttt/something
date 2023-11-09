use crate::error::ParseError;

pub type ParseResult<'a, T>
where
    T: 'a,
= Result<T, ParseError<'a>>;
