use crate::{
    error::{EndOfTokens, ParseError},
    lexer::token::Token,
    prelude::ParseResult,
};
pub mod item;
pub mod nodes;
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Parser<'a> {
    pub src: &'a str,
    pub tokens: &'a [Token<'a>],
    /// The index of the current token
    pub current: usize,
}

impl<'a> Parser<'a> {
    pub fn step<'b: 'a, T, F>(&mut self, F: F) -> ParseResult<'b, T>
    where
        F: FnOnce(&mut Parser) -> ParseResult<'b, T>,
    {
        let start = self.current;
        let tmp = F(self);
        match tmp {
            ParseResult::Err(err) => {
                self.current = start;
                ParseResult::Err(err)
            }
            ParseResult::Ok(ok) => ParseResult::Ok(ok),
        }
    }
    pub fn advance<'b: 'a>(&mut self) -> Result<&'b Token<'a>, ParseError<'b>> {
        if self.current > self.tokens.len() {
            Err(ParseError::EndOfTokens(EndOfTokens {}))
        } else {
            self.current += 1;
            Ok(unsafe { self.tokens.get_unchecked(self.current) })
        }
    }
    pub fn peek<'b: 'a>(&mut self) -> Result<&'b Token<'a>, ParseError<'b>> {
        if self.current > self.tokens.len() {
            Err(ParseError::EndOfTokens(EndOfTokens {}))
        } else {
            Ok(unsafe { self.tokens.get_unchecked(self.current) })
        }
    }
}
