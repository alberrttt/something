pub mod binary;
pub mod identifier;
use identifier::Identifier;
use parm_ast::{
    lexer::token::StringLiteral,
    parser::nodes::{expression::Boolean, statement::use_stmt::Number},
};

use crate::{
    ty::{Type, TypeRef},
    typechecker::Typechecker,
};
pub mod struct_expression;
#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a, 'b> {
    Identifier(Identifier<'a, 'b>),
    StringLiteral(&'b StringLiteral<'a>),
    Number(&'b Number<'a>),
    Boolean(&'b Boolean<'a>),
}

impl<'a, 'b> Expression<'a, 'b> {
    pub fn check_ast(expression: &'b parm_ast::prelude::Expression<'a>) -> Self {
        match &expression {
            parm_ast::prelude::Expression::StringLit(string_literal) => {
                Expression::StringLiteral(string_literal)
            }
            parm_ast::prelude::Expression::Number(number) => Expression::Number(number),
            parm_ast::prelude::Expression::Boolean(boolean) => Expression::Boolean(boolean),
            expression => todo!("{:?}", expression),
        }
    }
    pub fn get_ty(&self) -> TypeRef<'a, 'b> {
        match self {
            Expression::Identifier(identifier) => identifier.symbol.inner.borrow().ty.clone(),
            Expression::StringLiteral(_) => Typechecker::STRING_LITERAL,
            Expression::Number(_) => Typechecker::NUMBER,
            Expression::Boolean(_) => Typechecker::BOOLEAN,
        }
    }
}
