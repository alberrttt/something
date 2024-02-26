pub mod binary;
pub mod call;
pub mod identifier;
use crate::prelude::*;
use parm_ast::{
    lexer::token::StringLiteral,
    parser::nodes::{expression::Boolean, path::SimpleSegment, statement::use_stmt::Number},
    prelude,
};

use crate::{traits::Check, ty::Type, typechecker::Typechecker};

use self::{struct_expression::StructExpression, traits::TypeCheckResult};
pub mod struct_expression;
#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a, 'b> {
    Identifier(Identifier<'a, 'b>),
    StringLiteral(&'b StringLiteral<'a>),
    StructExpression(StructExpression<'a, 'b>),
    BinaryExpression(BinaryExpression<'a, 'b>),
    Number(&'b Number<'a>),
    Call(call::Call<'a, 'b>),
    Boolean(&'b Boolean<'a>),
    /// to be used for error recovery and handling only, the user should never be able to create this
    None,
}
impl<'a, 'b> Check<'a, 'b, Expression<'a, 'b>> for parm_ast::prelude::Expression<'a> {
    fn check(
        &'b self,
        tc: &mut Typechecker<'a, 'b>,
    ) -> TypeCheckResult<'a, 'b, Expression<'a, 'b>> {
        Ok(match &self {
            ast::Expression::StringLit(string_literal) => Expression::StringLiteral(string_literal),
            ast::Expression::Number(number) => Expression::Number(number),
            ast::Expression::Boolean(boolean) => Expression::Boolean(boolean),
            ast::Expression::StructExpression(struct_expression) => {
                Expression::StructExpression(struct_expression.check(tc)?)
            }
            ast::Expression::Identifier(identifier) => {
                Expression::Identifier(identifier.check(tc)?)
            }
            ast::Expression::BinaryExpression(bin) => Expression::BinaryExpression(bin.check(tc)?),
            ast::Expression::Group(_) => todo!(),
            ast::Expression::Call(call) => Expression::Call(call.check(tc)?),
            ast::Expression::Block(_) => todo!(),
            ast::Expression::If(_) => todo!(),
            ast::Expression::Path(path) => {
                let msg = format!(
                    "todo, path become its own expression and not coerced to an identifier {}:{}",
                    file!(),
                    line!(),
                );
                println!("{msg}");

                Expression::Identifier(
                    {
                        let tmp = path.first_segment();
                        let SimpleSegment::Identifier(identifier) = tmp else {
                            panic!()
                        };
                        identifier
                    }
                    .check(tc)?,
                )
            }
        })
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
            Expression::Call(call) => {
                todo!()
            }
            Expression::None => Type::Unknown { err: true },
        }
    }
}
