use parmesan_common::Spanned;
use parmesan_dev_macros::Spanned;

use crate::lexer::token::{Ident, Let};

#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct Variable<'a> {
    pub let_tkn: Let<'a>,
    pub ident: Ident<'a>,
}
