pub use crate::ast::prelude::*;

use self::tokenizer::Tkn;
#[derive(Debug, Clone, ParseTokens, ParseTokensDisplay)]
pub struct Path {
    pub segments: OmitTrailing<Ident, Tkn![:]>,
}

#[test]
fn test() {
    let (path, _): (Path, _) = ast!("standard:io:print");
    dbg!(path);
}
