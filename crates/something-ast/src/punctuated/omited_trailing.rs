use std::fmt::Debug;

use super::Punctuated;
use crate::prelude::*;
use something_dev_tools::ParseTokensDisplay;
use something_frontend_tokenizer::{traits::AppendTokens, Parse, ParsingDisplay};

#[derive(Debug, Clone)]
pub struct OmitTrailing<T, P>(pub Punctuated<T, P>);
impl<T, P> AppendTokens for OmitTrailing<T, P>
where
    T: AppendTokens,
    P: AppendTokens,
{
    fn append_tokens(&self, tokens: &mut Tokens)
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
    fn parse(input: &mut Tokens) -> Result<Self, ParseError> {
        Ok(Self(Punctuated::<T, P>::parse_without_trailing(input)?))
    }
}
#[test]
fn test() {
    let (ast, _): (OmitTrailing<Ident, Comma>, _) = ast!("a, b, c");
    dbg!(ast);
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
