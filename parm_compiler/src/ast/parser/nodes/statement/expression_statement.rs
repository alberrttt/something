use parm_common::Spanned;

use crate::ast::{lexer::token::SemiColon, parser::nodes::expression::Expression};

#[derive(Debug, Clone)]
pub struct ExpressionStatement<'a> {
    expression: Expression<'a>,
    semi_colon: SemiColon<'a>,
}
impl<'a> Spanned for ExpressionStatement<'a> {
    fn span(&self) -> parm_common::Span {
        (self.expression.span(), self.semi_colon.span).into()
    }
}
