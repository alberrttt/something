pub mod declaration;
pub mod delimiters;
pub mod expression;
pub mod prelude;
pub mod statement;
pub mod nodes {
    pub use super::declaration::{fnc::Function, var::Variable, Declaration};
    pub use super::delimiters::{Brace, Bracket, Paren};
    pub use super::expression::{block::Block, if_expr::IfExpr, Expression};
    pub use super::statement::Statement;
    pub use crate::{Ident, Literal};
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ast {
    pub nodes: Vec<TopLevelNode>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopLevelNode {
    Decl(),
}
