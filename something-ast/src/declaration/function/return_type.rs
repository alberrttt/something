use something_dev_tools::ParseTokens;
use something_frontend_tokenizer::{ident::Ident, tokens};

#[derive(Debug, Clone, ParseTokens)]
pub struct ReturnType {
    pub arrow: tokens::RightArrow,
    pub ty: Ident,
}
