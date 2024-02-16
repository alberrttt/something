pub mod binary;
pub mod identifier;
use identifier::Identifier;
use parm_ast::{
    lexer::token::StringLiteral,
    parser::nodes::{expression::Boolean, statement::use_stmt::Number},
};

use crate::{
    traits::Check,
    ty::{Type, TypeRef},
    typechecker::Typechecker,
};

use self::struct_expression::StructExpression;
pub mod struct_expression;
#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a, 'b> {
    Identifier(Identifier<'a, 'b>),
    StringLiteral(&'b StringLiteral<'a>),
    StructExpression(StructExpression<'a, 'b>),
    Number(&'b Number<'a>),
    Boolean(&'b Boolean<'a>),
}
impl<'a, 'b> Check<'a, 'b> for Expression<'a, 'b> {
    type Output = Self;
    type Ast = parm_ast::prelude::Expression<'a>;
    fn check(tc: &mut Typechecker<'a, 'b>, ast: &'b Self::Ast) -> Self::Output {
        match &ast {
            parm_ast::prelude::Expression::StringLit(string_literal) => {
                Expression::StringLiteral(string_literal)
            }
            parm_ast::prelude::Expression::Number(number) => Expression::Number(number),
            parm_ast::prelude::Expression::Boolean(boolean) => Expression::Boolean(boolean),
            parm_ast::prelude::Expression::StructExpression(struct_expression) => {
                Expression::StructExpression(StructExpression::check(tc, &struct_expression))
            }
            expression => todo!("{:#?}", expression),
        }
    }
}
impl<'a, 'b> Expression<'a, 'b> {
    pub fn get_ty(&self) -> Type<'a, 'b> {
        match self {
            Expression::Identifier(identifier) => identifier.symbol.inner.borrow().ty.clone(),
            Expression::StringLiteral(_) => Type::StringLiteral,
            Expression::Number(_) => Type::Number,
            Expression::Boolean(_) => Type::Boolean,
            Expression::StructExpression(struct_expression) => {
                struct_expression.symbol.inner.borrow().ty.clone()
            }
        }
    }
}
