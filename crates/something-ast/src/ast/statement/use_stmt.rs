use crate::ast::prelude::*;
use crate::tokenizer::token::{self, *};
use something_dev_tools::*;
use Macros::Tkn;
#[derive(Debug, Clone, ParseTokens, ParseTokensDisplay)]
pub struct Use {
    pub use_token: Tkn![Use],
    pub path: Path,
}
#[test]
fn test() {
    let (_, _): (Use, _) = ast!("d standard:io:print");
}
