use declaration::Declaration;
use expression::Expression;
use something_dev_tools::ParseTokens;
use something_frontend_tokenizer::tokens::Parse;
use something_frontend_tokenizer::Tokens;

pub struct Ast {
    pub nodes: Vec<Node>,
}
#[derive(Debug, ParseTokens)]
pub enum Node {
    Statement(statement::Statement),
}
pub use statement::Statement;
pub mod declaration;
pub mod delimiter;
pub mod expression;
pub mod punctuated;
pub mod statement;
