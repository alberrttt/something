use std::ops::Range;

use crate::lexer::token::Token;

use super::Parser;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseStream<'a> {
    pub tokens: &'a [Token<'a>],
    pub current: usize,
}

impl<'a> ParseStream<'a> {
    pub fn from_parse_stream(stream: &'a ParseStream<'a>, range: Range<usize>) -> Self {
        Self {
            tokens: &stream.tokens[range],
            current: 0,
        }
    }
    pub fn from_parser(parser: &'a Parser<'a>, range: Range<usize>) -> Self {
        Self {
            tokens: &parser.tokens[range],
            current: 0,
        }
    }
    pub fn at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
    pub fn advance(&mut self) -> Option<&'a Token<'a>> {
        if self.current >= self.tokens.len() {
            None
        } else {
            let token = unsafe { self.tokens.get_unchecked(self.current) };
            self.current += 1;
            Some(token)
        }
    }

    pub fn peek(&self) -> Option<&'a Token<'a>> {
        if self.current >= self.tokens.len() {
            None
        } else {
            Some(unsafe { self.tokens.get_unchecked(self.current) })
        }
    }
}
