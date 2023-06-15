use super::prelude::*;

/// this will turn into `AstProgramFile` in the ast portion
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenizerProgramFile {
    pub name: String,
    pub tokens: TokenStream,
}

impl TokenizerProgramFile {
    pub fn new(name: String, source_code: &str) -> Self {
        Self {
            name,
            tokens: TokenStream::from(source_code),
        }
    }
}
