use std::cell::UnsafeCell;

use crate::{
    error::{EndOfTokens, ErrorKind},
    lexer::token::Token,
    prelude::ParseResult,
    source_file::PreparsedSourceFile,
};

pub use self::parse_stream::ParseStream;
pub mod ast_displayer;
pub mod nodes;
pub mod parse_stream;
pub mod token_stream;
#[derive(Debug)]
pub struct Parser<'a> {
    pub src: &'a str,
    pub tokens: Vec<Token<'a>>,
    /// The index of the current token
    pub current: usize,
}
impl<'a> PartialEq for Parser<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.tokens == other.tokens && self.current == other.current && self.src == other.src
    }
}
impl<'a> Parser<'a> {
    pub fn stream<'b: 'a>(
        &'b self,
        src_file: &'a UnsafeCell<PreparsedSourceFile<'a>>,
    ) -> ParseStream<'b> {
        ParseStream {
            tokens: &self.tokens,
            current: self.current,
            src_file,
            panic: false,
            attributes: Default::default(),
        }
    }
    pub fn at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    pub fn step<T>(
        &mut self,
        closure: fn(&mut Parser<'a>) -> ParseResult<'a, T>,
    ) -> ParseResult<'a, T> {
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
    pub fn advance<'b>(&mut self) -> Result<&Token<'a>, ErrorKind<'b>> {
        if self.current > self.tokens.len() {
            Err(ErrorKind::EndOfTokens(EndOfTokens::default()))
        } else {
            self.current += 1;
            Ok(unsafe { self.tokens.get_unchecked(self.current) })
        }
    }
    pub fn peek<'b: 'a>(&self) -> Result<&'b Token<'a>, ErrorKind<'b>> {
        if self.current > self.tokens.len() {
            Err(ErrorKind::EndOfTokens(EndOfTokens::default()))
        } else {
            // as long as it compiles 🙂😀
            Ok(unsafe { ::std::mem::transmute(self.tokens.get_unchecked(self.current)) })
        }
    }
}
