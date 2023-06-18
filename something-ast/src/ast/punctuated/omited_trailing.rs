use std::fmt::Debug;

use Macros::Tkn;

use super::Punctuated;

use crate::tokenizer::prelude::*;

#[derive(Debug, Clone)]
pub struct OmitTrailing<T, P>(pub Punctuated<T, P>);
impl<T, P> AppendTokens for OmitTrailing<T, P>
where
    T: AppendTokens,
    P: AppendTokens,
{
    fn append_tokens(&self, tokens: &mut TokenStream)
    where
        Self: Sized,
    {
        for (item, punctuation) in self.0.iter() {
            item.append_tokens(tokens);
            if let Some(punctuation) = punctuation {
                punctuation.append_tokens(tokens);
            }
        }
    }
}
impl<T, P> From<OmitTrailing<T, P>> for Punctuated<T, P> {
    fn from(value: OmitTrailing<T, P>) -> Self {
        value.0
    }
}
impl<'a, T, P> From<&'a OmitTrailing<T, P>> for &'a Punctuated<T, P> {
    fn from(value: &'a OmitTrailing<T, P>) -> Self {
        &value.0
    }
}
impl<T, P> Parse for OmitTrailing<T, P>
where
    T: Debug + Parse,
    P: Debug + Parse,
{
    fn parse(input: &mut TokenStream) -> ParseResult<Self> {
        Ok(Self(Punctuated::<T, P>::parse_without_trailing(input)?))
    }
}
use crate::ast;
use crate::ast::prelude::*;
use crate::tokenizer::prelude::*;
#[test]
fn test() {
    let mut tokens = crate::tokenizer::TokenStream::from("a,b,c,d,");

    dbg!(match tokens.parse::<Punctuated<Ident, Tkn![,]>>() {
        Ok(value) => (value, tokens),
        Err(err) => {
            println!("{}", err);
            panic!();
        }
        Recoverable => todo!(),
    });
}
// create a test that should fail and is properly intergrated into rust's unit testing
#[test]
#[should_panic]
fn failure() {
    let (ast, _): (OmitTrailing<Ident, Comma>, _) = ast!("a, b, c,");
    dbg!(ast);
}
impl<T, P> ParsingDisplay for OmitTrailing<T, P>
where
    T: Debug + Parse,
    P: Debug + Parse,
{
    fn display(&self) -> String
    where
        Self: Sized,
    {
        todo!()
    }

    fn placeholder() -> String
    where
        Self: Sized,
    {
        todo!()
    }
}
