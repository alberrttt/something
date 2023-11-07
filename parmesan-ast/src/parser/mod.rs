use crate::{
    error::{EndOfTokens, ParseError},
    lexer::token::Token,
    result::PResult,
};
pub mod nodes;
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Parser<'a> {
    pub src: &'a str,
    pub tokens: &'a [Token<'a>],
    /// The index of the current token
    pub current: usize,
}

impl<'a> Parser<'a> {
    pub fn advance_1<'b: 'a>(&mut self) -> PResult<'b, &'b Token<'a>> {
        use PResult::*;
        if self.current > self.tokens.len() {
            Err(ParseError::EndOfTokens(EndOfTokens {}))
        } else {
            self.current += 1;
            Ok(unsafe { self.tokens.get_unchecked(self.current) })
        }
    }
    pub fn step<'b: 'a, T, F>(&mut self, F: F) -> PResult<'b, T>
    where
        F: FnOnce(&mut Parser) -> PResult<'b, T>,
    {
        let start = self.current;
        let tmp = F(self);
        match tmp {
            PResult::Err(err) => {
                self.current = start;
                PResult::Err(err)
            }
            PResult::Ok(ok) => PResult::Ok(ok),
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
