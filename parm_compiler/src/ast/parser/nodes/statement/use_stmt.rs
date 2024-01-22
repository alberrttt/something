pub use crate::ast::prelude::*;

use crate::ast::parser::nodes::path::SimplePath;

#[derive(Debug, Clone, PartialEq, Spanned, Parse, Tree)]
pub struct UseStatement<'a> {
    pub use_tkn: Use<'a>,
    pub path: SimplePath<'a>,
    pub semi: SemiColon<'a>,
}
