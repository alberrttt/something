use crate::attribute::Attribute;

use self::return_type::ReturnType;
use super::super::prelude::*;

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub modifiers: Option<Attribute>,
    pub fn_token: tokens::Fn,
    pub name: Ident,
    pub params: Parentheses<Vec<Ident>>,
    pub body: Braces<Vec<Node>>,
    pub return_type: Option<ReturnType>,
}
impl Parse for FunctionDeclaration {
    fn parse(input: &mut Tokens) -> Result<Self, Box<dyn std::error::Error>> {
        let modifiers = input.step(|input| input.parse()).ok();
        let fn_token = input.parse()?;

        let name = input.parse()?;
        let params = input.parse()?;
        let body = input.parse()?;
        let return_type = (input.parse()).ok();
        Ok(Self {
            modifiers,
            fn_token,
            name,
            params,
            body,
            return_type,
        })
    }
}
pub mod return_type;
