use std::path::Display;

use something_dev_tools::{ParseTokens, ParseTokensDisplay};
use something_frontend_tokenizer::{ident::Ident, tokens};

#[derive(Debug, Clone, ParseTokensDisplay, ParseTokens)]
pub struct ReturnType {
    pub arrow: tokens::RightArrow,
    pub ty: Ident,
}
impl std::fmt::Display for ReturnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.arrow, self.ty)
    }
}
use something_dev_tools::item_name;
item_name!(ReturnType, "return type");
