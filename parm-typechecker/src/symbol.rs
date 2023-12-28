use parm_ast::prelude::*;

use crate::types::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol<'a> {
    pub declaration: Option<SymbolDeclaration<'a>>,

    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolDeclaration<'a> {
    Function(Function<'a>),
    Struct(Struct<'a>),
    Variable(LetStmt<'a>),
    Param(Param<'a>),
}

impl<'a> SymbolDeclaration<'a> {
    pub fn name(&self) -> &'a Identifier {
        match self {
            SymbolDeclaration::Function(function) => &function.name,
            SymbolDeclaration::Struct(struct_) => &struct_.ident,
            SymbolDeclaration::Variable(variable) => &variable.ident,
            SymbolDeclaration::Param(param) => &param.name,
        }
    }
}
