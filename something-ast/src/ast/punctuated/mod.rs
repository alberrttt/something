mod omited_trailing;
use std::ops::Deref;

use crate::{ast, tokenizer::prelude::*};
use clap::error::Error;
pub use omited_trailing::*;
use Macros::Tkn;

// prolly need better error handling soon
#[derive(Debug, Clone)]
pub struct Punctuated<T, P>(pub Vec<(T, Option<P>)>);
impl<T, P> AppendTokens for Punctuated<T, P>
where
    Self: Sized,
    T: AppendTokens,
    P: AppendTokens,
{
    fn append_tokens(&self, tokens: &mut TokenStream) {
        for (t, p) in &self.0 {
            t.append_tokens(tokens);
            if let Some(p) = p {
                p.append_tokens(tokens);
            }
        }
    }
}
impl<T, P> Deref for Punctuated<T, P> {
    type Target = Vec<(T, Option<P>)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T, P> ParsingDisplay for Punctuated<T, P>
where
    T: ParsingDisplay,
    P: ParsingDisplay,
{
    fn display(&self) -> String {
        self.0
            .iter()
            .map(|(t, p)| {
                let mut tmp = t.display();
                if let Some(p) = p {
                    tmp.push_str(&p.display());
                }
                tmp
            })
            .collect::<String>()
    }
    fn placeholder() -> String {
        todo!()
    }
}
impl<T, P> Parse for Punctuated<T, P>
where
    T: Parse,
    P: Parse,
{
    fn parse(input: &mut TokenStream) -> ParseResult<Self> {
        Self::parse_terminated(input)
    }
}
impl<T, P> Punctuated<T, P>
where
    T: Parse,
    P: Parse,
{
    pub fn has_trailing(&self) -> bool {
        self.0.last().unwrap().1.is_some()
    }
    pub fn parse_without_trailing(input: &mut TokenStream) -> ParseResult<Self> {
        let mut vec = Vec::new();
        loop {
            let item = T::parse(input)?;
            if input.at_end() || input.is_empty() {
                vec.push((item, None));
                break;
            }
            let punct = P::parse(input)?;
            if input.at_end() || input.is_empty() {
                return Err(ParseError::ExpectedEnd(
                    (*input.previous().unwrap()).clone(),
                ));
            }
            vec.push((item, Some(punct)));
        }
        Ok(Self(vec))
    }
    pub fn parse_trailing(input: &mut TokenStream) -> ParseResult<Self> {
        let mut vec = Vec::new();
        loop {
            if input.at_end() || input.is_empty() {
                break;
            }
            let item = T::parse(input)?;

            vec.push((item, Some(P::parse(input)?)));
        }
        Ok(Self(vec))
    }
    pub fn parse_terminated(input: &mut TokenStream) -> ParseResult<Self> {
        let mut vec = Vec::new();
        loop {
            let item = match input.step(|input| T::parse(input)) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
                Recoverable => break,
                // this is a hack, but it works, it might cause problems later
            };

            let punct = if if input.peek().is_ok() {
                // this check might be jjanky but it works ( for now )
                input.peek().unwrap().is_closing_delimiter()
            } else {
                false
            } {
                break;
            } else {
                let parse = input.step(|f| P::parse(f));
                match parse {
                    Ok(punct) => Some(punct),
                    Err(err) => {
                        return Err(err);
                    }
                    Recoverable => todo!(),
                }
            };
            vec.push((item, punct));
        }
        Ok(Self(vec))
    }
}

#[test]
fn test() {
    let (_, _): (Punctuated<Ident, Tkn!(,)>, _) = ast!("a,b,c);");
}
