use std::fmt::Display;

use crate::tokenizer::{self, list::List, traits::AppendTokens, Parse};
use prelude::{Children, Declaration, FunctionDeclaration};
use something_dev_tools::{ParseTokens, ParseTokensDisplay};
#[derive(Debug, Clone, ParseTokens, ParseTokensDisplay, Default)]
pub struct Ast {
    pub nodes: List<TopLevelNode>,
}

#[derive(Debug, Clone, ParseTokens, ParseTokensDisplay)]
pub enum TopLevelNode {
    FunctionDeclaration(FunctionDeclaration),
}
impl AppendTokens for TopLevelNode {
    fn append_tokens(&self, tokens: &mut tokenizer::Tokens)
    where
        Self: Sized,
    {
        match self {
            TopLevelNode::FunctionDeclaration(function_declaration) => {
                function_declaration.append_tokens(tokens)
            }
        }
    }
}
impl Display for TopLevelNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TopLevelNode::FunctionDeclaration(function_declaration) => {
                write!(f, "{}", function_declaration)
            }
        }
    }
}
impl Children<TopLevelNode> for Ast {
    fn children(&self) -> std::slice::Iter<TopLevelNode> {
        self.nodes.iter()
    }
}
#[derive(Debug, ParseTokens, Clone, ParseTokensDisplay)]
pub enum Node {
    Statement(statement::Statement),
    Declaration(Declaration),
}
impl AppendTokens for Node {
    fn append_tokens(&self, tokens: &mut tokenizer::Tokens)
    where
        Self: Sized,
    {
        todo!();
    }
}
pub mod attribute;
pub mod declaration;
pub mod delimiter;
pub mod error {
    pub use crate::tokenizer::error::*;
}
pub mod expression;
pub mod path;
pub mod prelude;
pub mod punctuated;
pub mod statement;
pub mod traits;
impl From<&str> for Ast {
    fn from(value: &str) -> Self {
        let mut tokens = tokenizer::Tokens::from(value);
        match Ast::parse(&mut tokens) {
            Ok(ast) => ast,
            Err(err) => {
                println!("{}", err);
                panic!();
            }
        }
    }
}

#[macro_export]
macro_rules! ast {
    ($str: expr) => {{
        use $crate::tokenizer::Parse;
        let mut tokens = $crate::tokenizer::Tokens::from($str);
        match (&mut tokens).parse() {
            Ok(value) => (value, tokens),
            Err(err) => {
                println!("{}", err);
                panic!();
            }
        }
    }};
}
#[macro_export]
macro_rules! ident {
    ($str:tt, $span:expr) => {
        $crate::prelude::Ident {
            span: $span,
            name: $str.into(),
        }
    };
}