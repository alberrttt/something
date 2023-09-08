use std::fmt::Display;

use crate::tokenizer::{self, list::List, token::Token, traits::AppendTokens, Parse, TokenStream};
use prelude::{Children, Declaration, FunctionDeclaration};
use something_common::devprintln;
use something_dev_tools::{ParseTokens, ParseTokensDisplay};
impl AppendTokens for TopLevelNode {
    fn append_tokens(&self, tokens: &mut tokenizer::TokenStream)
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

#[derive(Debug, Clone, ParseTokensDisplay)]
pub enum Node {
    Statement(statement::Statement),
    Declaration(Declaration),
}
impl Parse for Node {
    fn parse(parser: &mut crate::parser::Parser) -> ParseResult<Self> {
        match parser.step(|parser| Parse::parse(parser)) {
            Ok(variant) => return Ok(Node::Statement(variant)),
            Err(err) => {
                loop {
                    if let Ok(Token::Semicolon(_)) = parser.peek() {
                        parser.advance();
                        break;
                    }
                    parser.advance();
                }

                return Err(err);
            }
            Recoverable => {}
        }
        match Parse::parse(parser) {
            Ok(variant) => return Ok(Node::Declaration(variant)),
            Err(err) => {
                return Err(err);
            }
            Recoverable => {}
        }
        Recoverable
    }
}
impl Parse for Box<Node> {
    fn parse(parser: &mut crate::parser::Parser) -> ParseResult<Self> {
        Ok(Box::new(Node::parse(parser)?))
    }
}
impl AppendTokens for Node {
    fn append_tokens(&self, _tokens: &mut tokenizer::TokenStream)
    where
        Self: Sized,
    {
        match self {
            Node::Statement(stmt) => stmt.append_tokens(_tokens),
            Node::Declaration(decl) => decl.append_tokens(_tokens),
        }
    }
}
pub mod attribute;
pub mod declaration;
pub mod delimiter;
pub mod error {
    pub use crate::error::*;
}
#[derive(Debug, Clone, ParseTokens, ParseTokensDisplay)]
pub enum TopLevelNode {
    FunctionDeclaration(FunctionDeclaration),
}
#[derive(Debug, Clone, ParseTokens, ParseTokensDisplay, Default)]
pub struct Ast {
    pub nodes: List<TopLevelNode>,
}
pub mod expression;
pub mod path;
pub mod prelude;
pub mod punctuated;
pub mod statement;
pub mod traits;
use crate::prelude::*;

#[macro_export]
macro_rules! ast {
    ($str: expr) => {{
        use $crate::prelude::*;
        let mut tokens = $crate::parser::Parser::new("test", $str);
        match (&mut tokens).parse() {
            Ok(value) => (value, tokens),
            Err(err) => {
                $crate::prelude::devprintln!("{}", err);
                panic!();
            }
            Recoverable => todo!(),
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
