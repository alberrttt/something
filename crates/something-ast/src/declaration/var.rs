use super::super::prelude::*;
#[derive(Debug, ParseTokensDisplay, Clone)]
pub struct VariableDeclaration {
    pub let_token: tokens::Let,
    pub name: Ident,
    pub type_annotation: Option<(Colon, Ident)>,
    pub equal: tokens::Equal,
    pub value: Expression,
    pub semicolon: tokens::Semicolon,
}
mod __variabledeclaration {
    use super::VariableDeclaration;
    use colored::Colorize;
    use something_frontend_tokenizer::prelude::ParseError;
    use something_frontend_tokenizer::Parse;
    use something_frontend_tokenizer::Tokens;
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
                    value: match Parse::parse(input) {
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
pub use __variabledeclaration::*;
use something_dev_tools::item_name;
item_name!(VariableDeclaration, "variable declaration");

#[test]
pub fn type_annotation() {
    let (var_decl, _): (VariableDeclaration, _) = crate::ast!("let a: i32 = 1;");
    dbg!(var_decl);
}
