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
    pub tokens: Vec<Token<'a>>,
    /// The index of the current token
    pub current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        let tokens = crate::lexer::Lexer::from(src).lex();

        Self {
            src,
            tokens,
            current: 0,
        }
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
