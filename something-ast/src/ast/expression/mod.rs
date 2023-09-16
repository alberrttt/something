use crate::prelude::*;

pub mod block;
pub mod if_expr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Block(block::Block),
    Lit(Literal),
    IfExpr(if_expr::IfExpr),
}
impl Node for Expression {
    fn parse(parser: &mut crate::Parser) -> ParseResult<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn span(&self) -> Span {
        todo!()
    }

    fn append_tokens(&self, to: &mut Vec<Token>) {
        todo!()
    }
}
