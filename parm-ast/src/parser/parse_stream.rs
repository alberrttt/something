use std::{cell::UnsafeCell, ops::Range};

use crate::{prelude::*, source_file::PreparsedSourceFile};

#[derive(Debug)]
pub struct ParseStream<'a> {
    pub tokens: &'a [Token<'a>],
    pub current: usize,
    pub src_file: &'a UnsafeCell<PreparsedSourceFile<'a>>,
    pub panic: bool,
}
impl<'a> PartialEq for ParseStream<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.tokens == other.tokens && self.current == other.current
    }
}
impl<'a> ParseStream<'a> {
    pub fn current_location_in_file(&self) -> usize {
        let src_file_tokens = &unsafe { &*self.src_file.get() }.parser.tokens;
        let location = src_file_tokens
            .windows(self.tokens.len())
            .position(|window| window == self.tokens)
            .unwrap();
        location
    }
    pub fn from_src_file(
        src_file: &'a UnsafeCell<PreparsedSourceFile<'a>>,
        range: Range<usize>,
    ) -> Self {
        Self {
            tokens: &unsafe { &*src_file.get() }.parser.tokens[range],
            current: 0,
            src_file,
            panic: false,
        }
    }
    pub fn previous(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.current - 1)
    }
    pub fn from_range(&self, range: Range<usize>) -> Self {
        Self {
            tokens: &self.tokens[range],
            current: 0,
            src_file: self.src_file,
            panic: false,
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
