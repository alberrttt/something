use crate::ast::prelude::*;

use super::block::Block;
#[derive(Debug, Clone, PartialEq, Tree, Spanned)]
pub struct IfExpr<'a> {
    pub if_tkn: If<'a>,
    pub condition: Box<Expression<'a>>,
    pub body: Block<'a>,
    pub else_branch: Option<ElseBranch<'a>>,
}
impl<'a> Node<'a> for IfExpr<'a> {
    fn parse(parse_stream: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let if_tkn = parse_stream.step(If::parse)?;
        parse_stream.panic = true;
        let condition = parse_stream.step(Expression::parse)?;
        let body = parse_stream.step(Block::parse)?;

        let mut else_branch = None;
        if let Ok(Token::Else(_)) = parse_stream.peek() {
            else_branch = Some(parse_stream.step(ElseBranch::parse)?);
        }

        parse_stream.panic = false;
        Ok(Self {
            if_tkn,
            condition: Box::new(condition),
            body,
            else_branch,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Tree, Spanned)]
pub struct ElseBranch<'a> {
    pub else_tkn: Else<'a>,
    pub body: Block<'a>,
}
impl<'a> Node<'a> for ElseBranch<'a> {
    fn parse(parse_stream: &mut ParseStream<'a>) -> Result<Self, Box<ParseError<'a>>>
    where
        Self: Sized,
    {
        let else_tkn = parse_stream.step(|parse_stream| Else::parse(parse_stream))?;

        let body = Block::parse(parse_stream)?;

        Ok(Self { else_tkn, body })
    }
}
