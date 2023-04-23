use something_dev_tools::ParseTokens;
use something_frontend_tokenizer::tokens::{self, Ident, Parse};

#[derive(Debug, ParseTokens)]
pub struct FunctionDeclaration {
    pub fn_token: tokens::Fn,
    pub name: Ident,
    pub params: (), // todo
    pub body: (),   // todo
}
