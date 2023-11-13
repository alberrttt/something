use crate::{
    error::{EndOfTokens, ParseError},
    lexer::token::Token,
    prelude::ParseResult,
};

pub use self::parse_stream::ParseStream;

pub mod item;
pub mod nodes;
pub mod parse_stream;
#[derive(Debug, Clone, PartialEq)]
pub struct Parser<'a> {
    pub src: &'a str,
    pub tokens: Vec<Token<'a>>,
    pub stream: ParseStream<'a>,
    /// The index of the current token
    pub current: usize,
}

impl<'a> Parser<'a> {
    pub fn stream<'b: 'a>(&'b self) -> ParseStream<'b> {
        ParseStream {
            tokens: &self.tokens,
            current: self.current,
        }
    }
    pub fn at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
    pub fn new(src: &'a str) -> Self {
        let tokens = crate::lexer::Lexer::from(src).lex();
        let reference =
            unsafe { std::mem::transmute::<&[Token<'a>], &'a [Token<'a>]>(tokens.as_ref()) };
        Self {
            src,
            tokens,
            stream: ParseStream {
                tokens: reference,
                current: 0,
            },
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
