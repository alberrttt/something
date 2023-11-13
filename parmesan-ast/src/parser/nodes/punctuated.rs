use std::marker::PhantomData;

use parmesan_common::Spanned;

use crate::{parser::Parser, prelude::ParseResult, traits::Node};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Punctuated<T, P> {
    pub inner: Vec<(T, P)>,
    pub last: Option<Box<T>>,
}
impl<T: Spanned, P: Spanned> Spanned for Punctuated<T, P> {
    fn span(&self) -> parmesan_common::Span {
        let start = match self.inner.first() {
            Some((node, _)) => node.span(),
            None => match &self.last {
                Some(node) => node.span(),
                None => return parmesan_common::Span::default(),
            },
        };
        let end = match &self.last {
            Some(node) => node.span(),
            None => panic!(),
        };

        (start, end).into()
    }
}
impl<'a, T: Node<'a>, P: Node<'a>> Punctuated<T, P> {
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

    pub fn parse_terminated(parser: &mut crate::parser::ParseStream<'a>) -> ParseResult<'a, Self>
    where
        T: Default,
        P: Default,
    {
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
        }
        Ok(punctuated)
    }
}
