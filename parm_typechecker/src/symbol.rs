use std::rc::Rc;

use parm_ast::prelude::*;

use crate::types::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol<'a, 'b: 'a> {
    pub declaration: Option<SymbolDeclaration<'a, 'b>>,

    pub ty: Rc<Type>,
}

#[derive(Clone, PartialEq)]
pub enum SymbolDeclaration<'a, 'b: 'a> {
    Function(&'b Function<'a>),
    Struct(&'b Struct<'a>),
    Variable(&'b LetStatement<'a>),
    Param(&'b Param<'a>),
    None,
}
impl std::fmt::Debug for SymbolDeclaration<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Function(arg0) => f.debug_tuple("Function").finish(),
            Self::Struct(arg0) => f.debug_tuple("Struct").finish(),
            Self::Variable(arg0) => f.debug_tuple("Variable").finish(),
            Self::Param(arg0) => f.debug_tuple("Param").finish(),
            Self::None => write!(f, "None"),
        }
    }
}
impl<'a, 'b: 'a> SymbolDeclaration<'a, 'b> {
    pub fn name(&self) -> &'a Identifier {
        match self {
            SymbolDeclaration::Function(function) => &function.name,
            SymbolDeclaration::Struct(struct_) => &struct_.ident,
            SymbolDeclaration::Variable(variable) => &variable.ident,
            SymbolDeclaration::Param(param) => &param.name,
            SymbolDeclaration::None => &COMPILER_IDENT,
        }
    }
}
