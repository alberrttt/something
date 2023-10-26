use crate::{
    error::{EndOfTokens, ParseError},
    lexer::token::Token,
};
pub mod nodes;
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Parser<'a> {
    src: &'a str,
    tokens: &'a [Token<'a>],
    /// The index of the current token
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn peek<'b: 'a>(&mut self) -> Result<&'b Token<'a>, ParseError<'b>> {
        if self.current > self.tokens.len() {
            Err(ParseError::EndOfTokens(EndOfTokens {}))
        } else {
            Ok(unsafe { self.tokens.get_unchecked(self.current) })
        }
    }

}
