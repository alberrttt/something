use something_dev_tools::{ParseTokens, ParseTokensDisplay};
use something_frontend_tokenizer::list::List;
#[derive(Debug, Clone, ParseTokens, ParseTokensDisplay)]
pub struct Ast {
    pub nodes: List<Node>,
}
#[derive(Debug, ParseTokens, Clone, ParseTokensDisplay)]
pub enum Node {
    Statement(statement::Statement),
}
pub mod attribute;
pub mod declaration;
pub mod delimiter;
pub mod error;
pub mod expression;
pub mod prelude;
pub mod punctuated;
pub mod statement;
