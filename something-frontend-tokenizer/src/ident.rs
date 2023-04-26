use std::error::Error;

use crate::{
    tokens::{Parse, Span},
    Token, Tokens,
};

#[derive(Clone, Debug)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}
impl Parse for Ident {
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn Error>> {
        let token = input.advance();
        if let Some(Token::Ident(token)) = token {
            Ok(token.clone())
        } else {
            Err(format!("Expected Ident, got {:?}", token).into())
        }
    }
}
