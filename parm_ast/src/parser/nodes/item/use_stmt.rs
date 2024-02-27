pub use crate::prelude::*;

use crate::parser::nodes::path::SimplePath;

#[derive(Debug, Clone, PartialEq, Spanned, Parse, Tree)]
pub struct UseStatement<'a> {
    pub use_tkn: UseKw<'a>,
    pub path: SimplePath<'a>,
    pub semi: SemiColon<'a>,
}