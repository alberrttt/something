
pub mod declaration;
pub mod prelude;
pub mod statement;
pub mod expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ast {
    pub nodes: Vec<TopLevelNode>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopLevelNode {
    Decl(),
}
