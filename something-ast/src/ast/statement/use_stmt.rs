use crate::ast::prelude::*;


use Macros::Tkn;
#[derive(Debug, Clone, ParseTokens, ParseTokensDisplay)]
pub struct Use {
    pub use_token: Tkn![Use],
    pub path: Path,
}
#[test]
fn test() {
    let (_, _): (Use, _) = ast!("use standard:io:print");
}
