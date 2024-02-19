use parm_ast::lexer::token::BinaryOperator;

use crate::prelude::*;

use self::traits::TypeCheckResult;

use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression<'a, 'b> {
    pub left: Box<Expression<'a, 'b>>,
    pub operator: AST<&'b BinaryOperator<'a>>,
    pub right: Box<Expression<'a, 'b>>,
}

impl<'a, 'b> Check<'a, 'b, BinaryExpression<'a, 'b>> for ast::BinaryExpression<'a> {
    fn check(
        &'b self,
        tc: &mut Typechecker<'a, 'b>,
    ) -> TypeCheckResult<'a, 'b, BinaryExpression<'a, 'b>> {
        let left = Box::new(self.left.check(tc)?);
        let right = Box::new(self.right.check(tc)?);
        Ok(BinaryExpression {
            left,
            operator: AST(&self.operator),
            right,
        })
    }
}
