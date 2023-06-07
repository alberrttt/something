use super::prelude::*;

/// this will turn into `AstProgramFile` in the ast portion
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenizerProgramFile {
    pub name: String,
    pub tokens: Tokens,
}

impl TokenizerProgramFile {
    pub fn new(name: String, source_code: &str) -> Self {
        Self {
            name,
            tokens: Tokens::from(source_code),
        }
    }
}
