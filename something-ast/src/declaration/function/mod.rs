use something_dev_tools::ParseTokensDisplay;
use something_frontend_tokenizer::{list::List, Parse};

use crate::attribute::Attribute;

use self::return_type::ReturnType;
use super::super::prelude::*;

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub modifiers: Option<Attribute>,
    pub fn_token: tokens::Fn,
    pub name: Ident,
    pub params: Parentheses<List<Ident>>,
    pub body: Braces<List<Node>>,
    pub return_type: Option<ReturnType>,
}
impl ParsingDisplay for FunctionDeclaration {
    fn display(&self) -> String
    where
        Self: Sized,
    {
        let mut result = String::new();
        if let Some(modifiers) = &self.modifiers {
            result.push_str(&modifiers.display());
            result.push(' ');
        }
        result.push_str(&self.fn_token.display());
        result.push(' ');
        result.push_str(&self.name.display());
        result.push_str(&self.params.display());
        use std::fmt::Write;

        writeln!(result, " {{").unwrap();
        self.body
            .1
            .iter()
            .for_each(|f| writeln!(result, "  {}", f.display()).unwrap());
        writeln!(result, "}}").unwrap();
        if let Some(return_type) = &self.return_type {
            result.push_str(&return_type.display());
        }
        result
    }

    fn placeholder() -> String
    where
        Self: Sized,
    {
        "fn <name>([...]) {<body...>} -> <return_type>".into()
    }
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
