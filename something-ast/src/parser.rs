use std::{
    fmt::Formatter,
    ops::{Deref, DerefMut},
};

use crate::{
    ast::Ast,
    prelude::ParseResult,
    tokenizer::{traits::Node, TokenStream},
};

#[derive(Debug)]
pub struct Parser<'a> {
    pub ast: Option<Ast>,
    pub source: &'a str,
    pub file_name: &'a str,
    pub token_stream: TokenStream,
}
impl<'a> From<TokenStream> for Parser<'a> {
    fn from(token_stream: TokenStream) -> Self {
        Self {
            ast: None,
            source: "",
            file_name: "",
            token_stream,
        }
    }
}
impl Deref for Parser<'_> {
    type Target = TokenStream;

    fn deref(&self) -> &Self::Target {
        &self.token_stream
    }
}
impl DerefMut for Parser<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.token_stream
    }
}
impl<'a> Parser<'a> {
    pub fn new(file_name: &'a str, source: &'a str) -> Self {
        Self {
            ast: None,
            source,
            file_name,
            token_stream: TokenStream::from(source),
        }
    }
    pub fn parse<T>(&mut self) -> ParseResult<T>
    where
        T: Node,
        T: Clone + std::fmt::Debug + Clone,
    {
        T::parse(self)
    }
    pub fn step<R>(&mut self, F: impl FnOnce(&mut Self) -> ParseResult<R>) -> ParseResult<R> {
        let starting = self.1;
        let stepped = F(self);
        match stepped {
            Ok(ok) => Ok(ok),
            Err(e) => {
                self.1 = starting;
                Err(e)
            }
        }
    }
}
#[derive(Debug)]
pub struct ErrorHandler<'a> {
    pub parse_errors: Vec<crate::error::ParseError>,
    pub panic_mode: bool,
    pub parser: &'a Parser<'a>,
}

impl<'a> ErrorHandler<'a> {
    fn write_errors(&self, fmter: &mut Formatter) -> std::fmt::Result {
        for error in &self.parse_errors {
            // error.write_error(self.parser, fmter)?;
        }
        Ok(())
    }
}

#[test]
fn test() {
    let source = "fn main() { print(\"Hello, world!\"); }";
    let file_name = "main.something";
    let mut parser = Parser::new(file_name, source);
}
