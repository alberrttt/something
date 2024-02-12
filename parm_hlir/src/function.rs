use std::rc::Rc;

use parm_ast::parser::nodes::statement::use_stmt::FunctionDeclaration;

use crate::{
    symbol::Symbol,
    ty::{function_ty::FunctionTy, Type, TypeRef},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Function<'a, 'b> {
    pub symbol: Symbol<'a, 'b>,

    pub statements: Vec<crate::statement::Statement<'a, 'b>>,
}
