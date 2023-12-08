use parm_common::Spanned;
use parm_dev_macros::Spanned;

use crate::prelude::{Colon, Identifier};

#[derive(Debug, Clone, Spanned)]
pub struct TypeName<'a> {
    inner: Identifier<'a>,
}

#[derive(Debug, Clone, Spanned)]
pub struct TypeAnnotation<'a> {
    pub colon: Colon<'a>,
    pub type_name: TypeName<'a>,
}
