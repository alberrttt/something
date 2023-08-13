mod omited_trailing;
use std::ops::Deref;

use crate::{ast, tokenizer::prelude::*};
use clap::error::Error;
pub use omited_trailing::*;
use Macros::Tkn;

// prolly need better error handling soon
#[derive(Debug, Clone)]
pub struct Punctuated<Item, Punctuation>(pub Vec<(Item, Option<Punctuation>)>);
impl<Item, Punctuation> AppendTokens for Punctuated<Item, Punctuation>
where
    Self: Sized,
    Item: AppendTokens,
    Punctuation: AppendTokens,
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
    fn parse(parser: &mut crate::parser::Parser) -> ParseResult<Self> {
        Self::parse_terminated(parser)
    }
}
impl<Item, Punctuation> Punctuated<Item, Punctuation>
where
    Item: Parse,
    Punctuation: Parse,
{
    pub fn has_trailing(&self) -> bool {
        self.0.last().unwrap().1.is_some()
    }
    pub fn parse_error_on_trailing(parser: &mut crate::parser::Parser) -> ParseResult<Self> {
        todo!();
        let mut vec = Vec::new();
        loop {
            let item = Item::parse(parser)?;
            if parser.at_end() || parser.is_empty() {
                vec.push((item, None));
                break;
            }
            let punct = Punctuation::parse(parser)?;
            if parser.at_end() || parser.is_empty() {
                return Err(ParseError::ExpectedEnd(
                    (*parser.previous().unwrap()).clone(),
                ));
            }
            vec.push((item, Some(punct)));
        }
        Ok(Self(vec))
    }
    pub fn parse_trailing(parser: &mut crate::parser::Parser) -> ParseResult<Self> {
        let mut vec = Vec::new();
        loop {
            if parser.at_end() || parser.is_empty() {
                break;
            }
            let item = Item::parse(parser)?;

            vec.push((item, Some(Punctuation::parse(parser)?)));
        }
        Ok(Self(vec))
    }
    pub fn parse_terminated(parser: &mut crate::parser::Parser) -> ParseResult<Self> {
        let mut vec = Vec::new();
        loop {
            let item = match parser.step(|parser| Item::parse(parser)) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
                Recoverable => break,
                // this is a hack, but it works, it might cause problems later
            };

            let should_stop = if parser.peek().is_ok() {
                // this check might be jjanky but it works ( for now )
                parser.peek().unwrap().is_closing_delimiter()
            } else {
                parser.at_end()
            };
            let punct = if should_stop {
                vec.push((item, None));
                break;
            } else {
                let parse = parser.step(|f| Punctuation::parse(f));

                match parse {
                    Ok(punct) => Some(punct),
                    Err(err) => None,
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
    let (_, _): (Punctuated<Ident, Tkn!(,)>, _) = ast!("a,b,c");
}
