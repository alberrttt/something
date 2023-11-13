use std::ops::Range;

use crate::{lexer::token::Token, prelude::ParseResult, error::{ParseError, EndOfTokens}};

use super::Parser;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseStream<'a> {
    pub tokens: &'a [Token<'a>],
    pub current: usize,
}

impl<'a> ParseStream<'a> {
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
    pub fn advance<'b>(&mut self) -> Result<&Token<'a>, ParseError<'b>> {
        if self.current > self.tokens.len() {
            Err(ParseError::EndOfTokens(EndOfTokens {}))
        } else {
            self.current += 1;
            Ok(unsafe { self.tokens.get_unchecked(self.current) })
        }
    }
    pub fn peek<'b: 'a>(&self) -> Result<&'b Token<'a>, ParseError<'b>> {
        if self.current > self.tokens.len() {
            Err(ParseError::EndOfTokens(EndOfTokens {}))
        } else {
            // as long as it compiles 🙂😀
            Ok(unsafe { ::std::mem::transmute(self.tokens.get_unchecked(self.current)) })
        }
    }
}
