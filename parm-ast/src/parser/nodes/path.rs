use parm_common::Spanned;
use parm_dev_macros::Spanned;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct SimplePath<'a> {
    pub prefix: Option<ColonColon<'a>>,
    pub segments: Punctuated<Ident<'a>, ColonColon<'a>>,
}
