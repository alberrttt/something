use declaration::Declaration;
use something_dev_tools::ParseTokens;
use something_frontend_tokenizer::tokens::Parse;
use something_frontend_tokenizer::Tokens;

pub struct Ast {
    pub nodes: Vec<Node>,
}
#[derive(Debug, ParseTokens)]
pub enum Node {
    Statement(Statement),
}
#[derive(Debug, ParseTokens)]
pub enum Statement {
    Declaration(Declaration),
}
pub mod declaration;
pub mod delimiter;
