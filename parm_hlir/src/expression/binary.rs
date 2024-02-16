use parm_ast::lexer::token::BinaryOperator;

use crate::prelude::*;

use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression<'a, 'b> {
    pub left: Box<Expression<'a, 'b>>,
    pub operator: AST<&'b BinaryOperator<'a>>,
    pub right: Box<Expression<'a, 'b>>,
}

impl<'a, 'b> Check<'a, 'b> for BinaryExpression<'a, 'b> {
    type Output = Self;
    type Ast = ast::BinaryExpression<'a>;

    fn check(tc: &mut Typechecker<'a, 'b>, ast: &'b Self::Ast) -> Self::Output {
        let left = Box::new(Expression::check(tc, &ast.left));
        let right = Box::new(Expression::check(tc, &ast.right));
        Self {
            left,
            operator: AST(&ast.operator),
            right,
        }
    }
}
