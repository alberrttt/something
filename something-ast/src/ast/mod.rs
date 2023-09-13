pub mod declaration;
pub mod expression;
pub mod prelude;
pub mod statement;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ast {
    pub nodes: Vec<TopLevelNode>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopLevelNode {
    Decl(),
}
