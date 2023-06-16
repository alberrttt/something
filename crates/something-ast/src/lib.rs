#![feature(try_trait_v2)]




pub mod ast;
pub mod error;
pub mod tokenizer;

pub mod prelude {
    pub type ParseResult<T> = something_common::Result<T, ParseError>;
    pub use something_common::Result::*;

    use crate::error::ParseError;
}
#[macro_export]
macro_rules! tkn_recover {
    (eot $expr:expr) => {
        match $expr {
            Ok(x) => Ok(x),
            Err(_) | Recoverable => Recoverable,
        }
    };
}
#[macro_export]
/// imitates the `matches!` macro
macro_rules! peek_matches {
    ($self:ident, $($pat:pat_param)|+) => {
        match $self.peek() {
            Ok(token) => match token {
                $($pat)|+ => true,
                _ => false,
            },
            _ => false,
        }
    };
}

