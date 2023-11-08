use crate::{
    lexer::token::{self, Ident, Let, Token},
    parser::item::Item,
    traits::Node,
};
use parmesan_common::Spanned;
use parmesan_dev_macros::Spanned;
#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct Function<'a> {
    pub fn_tkn: token::FnKeyword<'a>,
    pub name: Ident<'a>,
    pub params: Vec<Ident<'a>>,
    pub body: Vec<Item<'a>>,
}
impl<'a> Node<'a> for Function<'a> {
    fn parse<'b: 'a>(
        parser: &'b mut crate::parser::Parser<'a>,
    ) -> Result<Self, crate::error::ParseError<'a>>
    where
        Self: Sized,
    {
        let fn_token = parser.step(|parser| token::FnKeyword::parse(parser).clone())?;
        let name = Ident::parse(parser)?;

        Ok(todo!())
    }
}
