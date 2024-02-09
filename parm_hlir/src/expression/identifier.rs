use crate::symbol::Symbol;
use parm_ast::prelude::Identifier as AstIdentifier;
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier<'a, 'b> {
    pub symbol: Symbol<'a, 'b>,
    pub ast_identifier: AstIdentifier<'a>,
}
