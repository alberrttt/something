use something_dev_tools::ParseTokens;

pub struct Ast {
    pub nodes: Vec<Node>,
}
#[derive(Debug, ParseTokens, Clone)]
pub enum Node {
    Statement(statement::Statement),
}
pub mod attribute;
pub mod declaration;
pub mod delimiter;
pub mod expression;
pub mod prelude;
pub mod punctuated;
pub mod statement;
