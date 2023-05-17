use prelude::{Children, Declaration, FunctionDeclaration};
use something_dev_tools::{ParseTokens, ParseTokensDisplay};
use something_frontend_tokenizer::{list::List, Parse};
#[derive(Debug, Clone, ParseTokens, ParseTokensDisplay)]
pub struct Ast {
    pub nodes: List<TopLevelNode>,
}
#[derive(Debug, Clone, ParseTokens, ParseTokensDisplay)]
pub enum TopLevelNode {
    FunctionDeclaration(FunctionDeclaration),
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

pub mod attribute;
pub mod declaration;
pub mod delimiter;
pub mod error;
pub mod expression;
pub mod prelude;
pub mod punctuated;
pub mod statement;
pub mod traits;
impl From<&str> for Ast {
    fn from(value: &str) -> Self {
        let mut tokens = something_frontend_tokenizer::Tokens::from(value);
        match Ast::parse(&mut tokens) {
            Ok(ast) => ast,
            Err(err) => {
                println!("{}", err);
                panic!();
            }
        }
    }
}
