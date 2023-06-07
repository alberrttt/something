use std::fmt::Display;

use crate::tokenizer::{list::List, traits::AppendTokens, Parse, ParsingDisplay};
use something_dev_tools::ParseTokensDisplay;

use crate::ast::{attribute::Attribute, expression::block::Block, punctuated::Punctuated};

use self::return_type::ReturnType;
use super::super::prelude::*;

#[derive(Debug, Clone, ParseTokensDisplay)]
pub struct FunctionDeclaration {
    // pub modifiers: Option<Attribute>,
    pub fn_token: Fn,
    pub name: Ident,
    pub params: Parentheses<Punctuated<(Ident, Ident), Comma>>,
    pub body: Block,
    pub return_type: ReturnType,
}
mod __functiondeclaration {
    use crate::tokenizer::prelude::*;
    use colored::Colorize;
    use std::fmt::{Display, Formatter};

    use super::FunctionDeclaration;
    impl Parse for FunctionDeclaration {
        fn parse(input: &mut Tokens) -> Result<Self, ParseError> {
            let tmp = input.step(|input| Parse::parse(input));
            match tmp {
                Ok(tmp) => Ok(Self {
                    fn_token: tmp,
                    name: Parse::parse(input)?,
                    params: Parse::parse(input)?,
                    body: Parse::parse(input)?,
                    return_type: Parse::parse(input)?,
                }),
                Err(err) => Err(err),
            }
        }
    }
    impl AppendTokens for FunctionDeclaration {
        fn append_tokens(&self, tokens: &mut Tokens) {
            self.fn_token.clone().append_tokens(tokens);
            self.name.clone().append_tokens(tokens);
            self.params.clone().append_tokens(tokens);
            self.body.clone().append_tokens(tokens);
            self.return_type.clone().append_tokens(tokens);
        }
    }
    impl Parse for Box<FunctionDeclaration> {
        fn parse(input: &mut Tokens) -> Result<Self, ParseError> {
            Ok(Box::new(FunctionDeclaration::parse(input)?))
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
                .map(|(i, (name, _))| { format!("{}: {},", name.0, name.1) })
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
use something_dev_tools::item_name;
mod __FunctionDeclaration {
    use super::FunctionDeclaration;
    use crate::tokenizer::prelude::Name;
    impl Name for FunctionDeclaration {
        fn name() -> &'static str {
            "function declaration"
        }
        fn named(&self) -> &'static str {
            "function declaration"
        }
    }
}
pub use __FunctionDeclaration::*;
pub mod return_type;
