use super::prelude::*;

#[derive(Debug, Clone, ParseTokens)]
pub struct Attribute {
    pub dollar: Dollar,
    pub brackets: Brackets<Vec<Ident>>,
    pub colon: Colon,
}
