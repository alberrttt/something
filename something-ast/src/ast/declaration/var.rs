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
    fn parse(parser: &mut crate::parser::Parser) -> ParseResult<Self> {
        let tmp = parser.step(|parser| Parse::parse(parser));
        match tmp {
            Ok(tmp) => Ok(Self {
                let_token: tmp,
                name: Parse::parse(parser)?,
                type_annotation: Parse::parse(parser)?,
                equal: Parse::parse(parser)?,
                expression: Parse::parse(parser)?,
                semicolon: Parse::parse(parser)?,
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
