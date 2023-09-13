use crate::{
    prelude::{token, Node},
    Span,
};

use super::Expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfExpr {
    pub if_: token::If,
    pub condition: Box<Expression>,
    pub then_branch: Box<Expression>,
    pub else_branch: Option<(token::Else, Box<Expression>)>,
}
impl Node for IfExpr {
    fn parse(parser: &mut crate::Parser) -> crate::ParseResult<Self> {
        todo!()
    }
    fn span(&self) -> crate::Span {
        Span {
            start: self.if_.span.start,
            end: self
                .else_branch
                .as_ref()
                .map(|(_, expr)| expr.span().end)
                .unwrap_or_else(|| self.then_branch.span().end),
            line: self.if_.span.line,
            line_start: self.if_.span.line_start,
        }
    }
}
