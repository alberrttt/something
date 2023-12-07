use std::ops::Range;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseStream<'a> {
    pub tokens: &'a [Token<'a>],
    pub current: usize,
}

impl<'a> ParseStream<'a> {
    pub fn previous(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.current - 1)
    }
    pub fn from_range(&self, range: Range<usize>) -> Self {
        Self {
            tokens: &self.tokens[range],
            current: 0,
        }
    }
    #[track_caller]
    pub fn step<T>(&mut self, closure: fn(&mut Self) -> ParseResult<'a, T>) -> ParseResult<'a, T> {
        let start = self.current;
        let result = closure(self);
        match result {
            Ok(ok) => Ok(ok),
            Err(err) => {
                self.current = start;
                Err(err)
            }
        }
    }
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
    pub fn advance<'b>(&mut self) -> ParseResult<'b, &'b Token<'a>> {
        match self.tokens.get(self.current) {
            Some(some) => {
                self.current += 1;
                Ok(some)
            }
            None => Err(ParseError::new(
                ErrorKind::EndOfTokens(EndOfTokens { expected: None }),
                self.tokens,
            )),
        }
    }
    pub fn peek<'b: 'a>(&self) -> ParseResult<'a, &'b Token<'a>> {
        match self.tokens.get(self.current) {
            Some(some) => Ok(some),
            None => Err(ParseError::new(
                ErrorKind::EndOfTokens(EndOfTokens { expected: None }),
                self.tokens,
            )),
        }
    }
}
