pub use crate::prelude::*;

#[derive(Debug, Clone, ParseTokens, ParseTokensDisplay)]
pub struct Path {
    pub segments: OmitTrailing<Ident, Slash>,
}

#[test]
fn test() {
    let (path, _): (Path, _) = ast!("top_level_module/function");
    dbg!(path);
}
