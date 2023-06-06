use something_frontend_tokenizer::{list::List, traits::AppendTokens};

use super::prelude::*;

#[derive(Debug, Clone, ParseTokensDisplay, ParseTokens)]
pub struct Attribute {
    pub dollar: Dollar,
    pub brackets: Brackets<List<Ident>>,
    pub colon: Colon,
}
