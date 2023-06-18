

use crate::tokenizer::{
    ident::Ident,
    prelude::{Macros::Tkn},
};
use something_dev_tools::{ParseTokens, ParseTokensDisplay};

#[derive(Debug, Clone, ParseTokensDisplay, ParseTokens)]
pub struct ReturnType {
    pub arrow: Tkn![->],
    pub ty: Ident,
}
impl std::fmt::Display for ReturnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.arrow, self.ty)
    }
}
use something_dev_tools::item_name;
item_name!(ReturnType, "return type");
