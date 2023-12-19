use crate::prelude::*;
pub type Args<'a> = Paren<'a, Punctuated<Expression<'a>, Comma<'a>>>;
#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct Call<'a> {
    pub callee: Box<Expression<'a>>,
    pub arguments: Args<'a>,
}

impl<'a> Node<'a> for Call<'a> {
    fn parse(parser: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        let callee = dbg!(Expression::parse(parser)?);
        let arguments = Paren::parse(parser)?;
        Ok(Self {
            callee: Box::new(callee),
            arguments,
        })
    }
}
impl<'a> Call<'a> {
    pub fn args(parser: &mut ParseStream<'a>) -> ParseResult<'a, Args<'a>> {
        Paren::parse(parser)
    }
}
