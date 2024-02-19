pub mod binary;
pub mod identifier;
use crate::prelude::*;
use parm_ast::{
    lexer::token::StringLiteral,
    parser::nodes::{expression::Boolean, statement::use_stmt::Number},
    prelude,
};

use crate::{traits::Check, ty::Type, typechecker::Typechecker};

use self::struct_expression::StructExpression;
pub mod struct_expression;
#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a, 'b> {
    Identifier(Identifier<'a, 'b>),
    StringLiteral(&'b StringLiteral<'a>),
    StructExpression(StructExpression<'a, 'b>),
    BinaryExpression(BinaryExpression<'a, 'b>),
    Number(&'b Number<'a>),
    Boolean(&'b Boolean<'a>),
}
impl<'a, 'b> Check<'a, 'b> for Expression<'a, 'b> {
    type Output = Self;
    type Ast = parm_ast::prelude::Expression<'a>;
    fn check(tc: &mut Typechecker<'a, 'b>, ast: &'b Self::Ast) -> Self::Output {
        match &ast {
            ast::Expression::StringLit(string_literal) => Expression::StringLiteral(string_literal),
            ast::Expression::Number(number) => Expression::Number(number),
            ast::Expression::Boolean(boolean) => Expression::Boolean(boolean),
            ast::Expression::StructExpression(struct_expression) => {
                Expression::StructExpression(StructExpression::check(tc, struct_expression))
            }
            ast::Expression::Identifier(identifier) => {
                Expression::Identifier(Identifier::check(tc, identifier))
            }
            ast::Expression::BinaryExpression(bin) => {
                Expression::BinaryExpression(BinaryExpression::check(tc, bin))
            }
            ast::Expression::Group(_) => todo!(),
            ast::Expression::Call(_) => todo!(),
            ast::Expression::Block(_) => todo!(),
            ast::Expression::If(_) => todo!(),
        }
    }
}
impl<'a, 'b> Expression<'a, 'b> {
    pub fn get_ty(&self) -> Type<'a, 'b> {
        match self {
            Expression::BinaryExpression(binary_expression) => {
                todo!()
            }
            Expression::Identifier(identifier) => identifier.symbol.inner.borrow().ty.clone(),
            Expression::StringLiteral(_) => Type::StringLiteral,
            Expression::Number(_) => Type::Int(ty::IntTy::Ambiguous),
            Expression::Boolean(_) => Type::Boolean,
            Expression::StructExpression(struct_expression) => {
                struct_expression.symbol.inner.borrow().ty.clone()
            }
        }
    }
}
