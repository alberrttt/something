use Macros::Tkn;

use crate::tokenizer::{list::List, traits::AppendTokens};

use super::prelude::*;

#[derive(Debug, Clone, ParseTokensDisplay, ParseTokens)]
pub struct Attribute {
    pub dollar: Tkn![$],
    pub brackets: Brackets<List<Ident>>,
    pub colon: Tkn![:],
}
