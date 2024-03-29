use std::fmt::{Debug, Display};

use crate::tokenizer::ParsingDisplay;
use something_dev_tools::ParseTokensDisplay;

use crate::ast::{expression::block::Block, punctuated::Punctuated};

use self::return_type::ReturnType;
use super::super::prelude::*;
use crate::tokenizer::token::Macros::Tkn;
#[derive(Clone, ParseTokensDisplay)]
pub struct FunctionDeclaration {
    // pub modifiers: Option<Attribute>,
    pub fn_token: Tkn![Fn],
    pub name: Ident,
    pub params: Paren<Punctuated<(Ident, Ident), Tkn![,]>>,
    pub body: Block,
    pub return_type: ReturnType,
}
// write a test
#[test]
fn test() {
    let (function_declaration, _): (Declaration, _) = ast!(
        "fn x(number x, number y) { 
        let z: number = y + x;
    } -> void"
    );
    assert!(matches!(function_declaration, Declaration::Function(_)));
    dbg!(function_declaration);
}

impl Debug for FunctionDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionDeclaration")
            .field("fn_token", &self.fn_token)
            .field("name", &self.name)
            .field("params", &self.params)
            .field("body", &self.body)
            .field("return_type", &self.return_type)
            .finish()
    }
}
mod __functiondeclaration {
    use crate::tokenizer::prelude::*;

    use super::FunctionDeclaration;
    impl Parse for FunctionDeclaration {
        fn parse(parser: &mut crate::parser::Parser) -> ParseResult<Self> {
            let tmp = parser.step(|parser| Parse::parse(parser));
            match tmp {
                Ok(tmp) => Ok(Self {
                    fn_token: tmp,
                    name: Parse::parse(parser)?,
                    params: Parse::parse(parser)?,
                    body: Parse::parse(parser)?,
                    return_type: Parse::parse(parser)?,
                }),
                Err(err) => Err(err),
            }
        }
    }
    impl AppendTokens for FunctionDeclaration {
        fn append_tokens(&self, tokens: &mut TokenStream) {
            self.fn_token.clone().append_tokens(tokens);
            self.name.clone().append_tokens(tokens);
            self.params.clone().append_tokens(tokens);
            self.body.clone().append_tokens(tokens);
            self.return_type.clone().append_tokens(tokens);
        }
    }
    impl Parse for Box<FunctionDeclaration> {
        fn parse(parser: &mut crate::parser::Parser) -> ParseResult<Self> {
            Ok(Box::new(FunctionDeclaration::parse(parser)?))
        }
    }
}
pub use __functiondeclaration::*;
impl Display for FunctionDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // if let Some(modifiers) = &self.modifiers {
        //     write!(f, "{} ", modifiers)?;
        // }
        write!(
            f,
            "{} {}{}",
            self.fn_token,
            self.name,
            self.params
                .iter()
                .enumerate()
                .map(|(_i, (name, _))| { format!("{}: {},", name.0, name.1) })
                .collect::<String>()
        )?;
        write!(f, "{}", self.return_type)?;
        write!(
            f,
            "{}",
            self.body
                .iter()
                .map(|f| { f.display() })
                .collect::<String>()
        )
    }
}

pub mod return_type;
