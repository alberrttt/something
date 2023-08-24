use std::fmt::format;

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
        let tmp: ParseResult<Tkn!(Let)> = parser.step(|parser| Parse::parse(parser));
        match tmp {
            Ok(tmp) => {
                let fields = parser.step(|parser| {
                    let name = Parse::parse(parser)?;
                    let type_annotation = Parse::parse(parser)?;
                    let equal = Parse::parse(parser)?;
                    let expression = Parse::parse(parser)?;
                    let semicolon = Parse::parse(parser)?;

                    Ok((name, type_annotation, equal, expression, semicolon))
                });
                let fields = match fields {
                    Ok(ok) => ok,
                    Err(err) => {
                        // recover by consuming all tokens until you reach the last, which is the semicolon
                        loop {
                            // devprintln!("{}", parser.peek()?);
                            if let Token::Semicolon(_) = parser.peek()? {
                                parser.advance()?;
                                break;
                            } else {
                                parser.advance();
                            }
                        }

                        something_common::devprintln!("recovered");
                        return Err(err);
                    }
                    Recoverable => return Recoverable,
                };
                let (name, type_annotation, equal, expression, semicolon) = fields;
                Ok(Self {
                    let_token: tmp,
                    name,
                    type_annotation,
                    equal,
                    expression,
                    semicolon,
                })
            }
            Err(err) => Err(err),
            Recoverable => Recoverable,
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
