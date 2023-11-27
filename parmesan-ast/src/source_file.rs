use crate::prelude::*;
use std::path::PathBuf;

use crate::{
    lexer::Lexer,
    parser::{item::Item, Parser},
    prelude::{Node, ParseError, ParseResult},
};

#[derive(Debug, Clone, PartialEq)]
pub struct PreparsedSourceFile<'a> {
    pub path: PathBuf,
    pub src: &'a str,
    pub lexer: Lexer<'a>,
    pub parser: Parser<'a>,
}
impl<'a> PreparsedSourceFile<'a> {
    pub fn parse(self) -> (SourceFile<'a>, Vec<ParseError<'a>>) {
        let mut stream: ParseStream<'_> = self.parser.stream.clone();
        let (ast, errors) =
            <Vec<Item<'a>> as Node<'a, (Vec<Item<'a>>, Vec<ParseError<'a>>)>>::parse(&mut stream);

        (
            SourceFile {
                preparsed: self,
                ast,
            },
            errors,
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SourceFile<'a> {
    pub preparsed: PreparsedSourceFile<'a>,
    pub ast: Vec<Item<'a>>,
}
