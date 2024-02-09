use parm_ast::parser::nodes::statement::use_stmt::FunctionDeclaration;

use crate::{
    symbol::{Symbol, SymbolRef},
    ty::Type,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Function<'a, 'b> {
    pub symbol: SymbolRef,
    pub return_ty: Type<'a, 'b>,
    pub statements: Vec<crate::statement::Statement<'a, 'b>>,
}

impl<'a, 'b> Function<'a, 'b> {}
