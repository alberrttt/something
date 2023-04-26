use something_dev_tools::ParseTokens;
use something_frontend_tokenizer::{
    ident::Ident,
    tokens::{self, Colon, Parse, Token},
    Token,
};

use crate::delimiter::{Braces, Brackets, Parenthesis};
#[derive(Debug)]
pub struct FnModifiers {
    pub modifiers: Vec<Ident>,
    pub public: bool,
}
impl Parse for FnModifiers {
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
        let mut fn_modifiers = Self {
            modifiers: vec![],
            public: false,
        };
        loop {
            let peek = input.peek();
            if let Some(Token::Ident(_)) = peek {
                let ident: Ident = Parse::parse(input)?;
                if ident.name == "pub" {
                    fn_modifiers.public = true;
                }
                fn_modifiers.modifiers.push(ident);
            } else {
                break;
            }
        }
        Ok(fn_modifiers)
    }
}
#[derive(Debug, ParseTokens)]
pub struct FunctionDeclaration {
    pub modifiers: Brackets<FnModifiers>,
    pub colon: Colon,
    pub fn_token: tokens::Fn,
    pub name: Ident,
    pub params: Parenthesis<Ident>, // todo
    pub body: Braces<()>,           // todo
}
