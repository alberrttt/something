use super::super::prelude::*;
#[derive(Debug, ParseTokensDisplay, Clone)]
pub struct VariableDeclaration {
    pub let_token: Let,
    pub name: Ident,
    pub type_annotation: Option<(Colon, Ident)>,
    pub equal: Equal,
    pub expression: Expression,
    pub semicolon: Semicolon,
}
impl AppendTokens for VariableDeclaration {
    fn append_tokens(&self, tokens: &mut Tokens)
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
mod __variabledeclaration {
    use super::VariableDeclaration;
    use crate::tokenizer::prelude::ParseError;
    use crate::tokenizer::Parse;
    use crate::tokenizer::Tokens;
    use colored::Colorize;
    use std::fmt::{Display, Formatter};
    impl Parse for VariableDeclaration {
        fn parse(input: &mut Tokens) -> Result<Self, ParseError> {
            let tmp = input.step(|input| Parse::parse(input));
            match tmp {
                Ok(tmp) => Ok(Self {
                    let_token: tmp,
                    name: match Parse::parse(input) {
                        Ok(ok) => ok,
                        Err(err) => {
                            println!("{}", err);
                            panic!()
                        }
                    },
                    type_annotation: match Parse::parse(input) {
                        Ok(ok) => ok,
                        Err(err) => {
                            println!("{}", err);
                            panic!()
                        }
                    },
                    equal: match Parse::parse(input) {
                        Ok(ok) => ok,
                        Err(err) => {
                            println!("{}", err);
                            panic!()
                        }
                    },
                    expression: match Parse::parse(input) {
                        Ok(ok) => ok,
                        Err(err) => {
                            println!("{}", err);
                            panic!()
                        }
                    },
                    semicolon: match Parse::parse(input) {
                        Ok(ok) => ok,
                        Err(err) => {
                            println!("{}", err);
                            panic!()
                        }
                    },
                }),
                Err(err) => Err(err),
            }
        }
    }
    impl Parse for Box<VariableDeclaration> {
        fn parse(input: &mut Tokens) -> Result<Self, ParseError> {
            Ok(Box::new(VariableDeclaration::parse(input)?))
        }
    }
}
use crate::tokenizer::traits::AppendTokens;
pub use __variabledeclaration::*;
use something_dev_tools::item_name;
item_name!(VariableDeclaration, "variable declaration");

#[test]
pub fn type_annotation() {
    let (var_decl, _): (VariableDeclaration, _) = crate::ast!("let a: i32 = 1;");
    dbg!(var_decl);
}
