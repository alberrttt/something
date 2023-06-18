use super::super::prelude::*;
use crate::prelude::*;
use crate::tokenizer::prelude::*;
#[derive(Debug, ParseTokensDisplay, Clone)]
pub struct VariableDeclaration {
    pub let_token: Tkn![Let],
    pub name: Ident,
    pub type_annotation: Option<(Tkn![:], Ident)>,
    pub equal: Tkn![=],
    pub expression: Expression,
    pub semicolon: Tkn![;],
}
impl Parse for VariableDeclaration {
    fn parse(input: &mut TokenStream) -> ParseResult<Self> {
        let tmp = input.step(|input| Parse::parse(input));
        match tmp {
            Ok(tmp) => Ok(Self {
                let_token: tmp,
                name: Parse::parse(input)?,
                type_annotation: Parse::parse(input)?,
                equal: Parse::parse(input)?,
                expression: Parse::parse(input)?,
                semicolon: Parse::parse(input)?,
            }),
            Err(_) | Recoverable => Recoverable,
        }
    }
}
impl AppendTokens for VariableDeclaration {
    fn append_tokens(&self, tokens: &mut TokenStream)
    where
        Self: Sized,
    {
        self.let_token.clone().append_tokens(tokens);
        self.name.clone().append_tokens(tokens);
        if let Some((colon, ident)) = &self.type_annotation {
            colon.clone().append_tokens(tokens);
            ident.clone().append_tokens(tokens);
        }
        self.equal.clone().append_tokens(tokens);

        self.expression.clone().append_tokens(tokens);
        self.semicolon.clone().append_tokens(tokens);
    }
}

use crate::{
    prelude::ParseResult,
    tokenizer::{traits::AppendTokens, Parse},
};
use something_dev_tools::item_name;
use Macros::Tkn;
item_name!(VariableDeclaration, "variable declaration");

#[test]
pub fn type_annotation() {
    let (var_decl, _): (VariableDeclaration, _) = crate::ast!("let a = 1;");
    dbg!(var_decl);
}
