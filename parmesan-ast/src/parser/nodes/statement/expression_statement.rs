use parmesan_common::Spanned;

use crate::{lexer::token::SemiColon, parser::nodes::expression::Expression, traits::Node};

#[derive(Debug, Clone)]
pub struct ExpressionStatement<'a> {
    expression: Expression<'a>,
    semi_colon: SemiColon<'a>,
}
impl<'a> Spanned for ExpressionStatement<'a> {
    fn span(&self) -> parmesan_common::Span {
        (self.expression.span(), self.semi_colon.span).into()
    }
}
