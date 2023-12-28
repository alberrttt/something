use std::slice::Iter;

use crate::prelude::*;
use parm_common::Spanned;

#[derive(Debug, Clone, PartialEq)]
pub struct Punctuated<T, P> {
    pub inner: Vec<(T, P)>,
    pub last: Option<Box<T>>,
}

impl<T, P> Default for Punctuated<T, P> {
    fn default() -> Self {
        Self {
            inner: Vec::new(),
            last: None,
        }
    }
}
impl<T: Spanned, P: Spanned> Spanned for Punctuated<T, P> {
    fn span(&self) -> parm_common::Span {
        let start = match self.inner.first() {
            Some((node, _)) => node.span(),
            None => match &self.last {
                Some(node) => node.span(),
                None => return parm_common::Span::default(),
            },
        };
        let end = match &self.last {
            Some(node) => node.span(),
            None => panic!(),
        };

        (start, end).into()
    }
}
impl<'a, T: Node<'a>, P: Node<'a>> Node<'a> for Punctuated<T, P> {
    /// the default parsing strategy is parse terminated
    fn parse(parser: &mut ParseStream<'a>) -> ParseResult<'a, Self>
    where
        Self: Sized,
    {
        Self::parse_terminated(parser)
    }
}
impl<'a, T: Node<'a>, P: Node<'a>> Punctuated<T, P> {
    pub fn iter(&self) -> Vec<&T> {
        let mut iter = self.inner.iter().map(|(t, _)| t).collect::<Vec<_>>();
        if let Some(last) = self.last.as_ref() {
            iter.push(last)
        }

        iter
    }

    pub fn new(inner: Vec<(T, P)>, last: Option<Box<T>>) -> Self {
        Self { inner, last }
    }

    pub fn push_value(&mut self, value: T) {
        assert!(self.last.is_none());
        self.last = Some(Box::new(value));
    }

    pub fn push_punc(&mut self, punct: P) {
        assert!(self.last.is_some());
        let last = self.last.take().unwrap();
        self.inner.push((*last, punct));
    }
    pub fn parse_terminated_expect(
        parser: &mut crate::parser::ParseStream<'a>,
    ) -> ParseResult<'a, Self> {
        let mut punctuated = Self::default();
        loop {
            if parser.at_end() {
                return T::parse(parser).map(|value| {
                    punctuated.push_value(value);
                    punctuated
                });
            }
            let value = T::parse(parser)?;
            punctuated.push_value(value);

            if parser.at_end() {
                break;
            }
            let punct = match P::parse(parser) {
                Ok(punct) => punct,
                Err(err) => {
                    break;
                }
            };
            punctuated.push_punc(punct);
        }
        Ok(punctuated)
    }

    pub fn parse_terminated(parser: &mut crate::parser::ParseStream<'a>) -> ParseResult<'a, Self> {
        let mut punctuated = Self::default();
        loop {
            if parser.at_end() {
                break;
            }

            let value = T::parse(parser)?;
            punctuated.push_value(value);
            if parser.at_end() {
                break;
            }
            let punct = match P::parse(parser) {
                Ok(punct) => punct,
                Err(_err) => {
                    break;
                }
            };
            punctuated.push_punc(punct);
        }
        Ok(punctuated)
    }
}

// #[test]
// fn parse_punctuated() {
//     let mut parser = Parser::new("a,b,c");
//     let punctuated =
//         Punctuated::<Identifier, Comma>::parse_terminated(&mut parser.stream()).unwrap();
//     dbg!(&punctuated);
// }

// #[test]
// fn parse_delimited() {
//     let mut parser = Parser::new("(a,b,c)");
//     let delimited = Paren::<Punctuated<Identifier, Comma>>::parse_manual(
//         &mut parser.stream(),
//         |parse_stream| Punctuated::<Identifier, Comma>::parse_terminated(parse_stream),
//     );
//     dbg!(&delimited);
// }
