use std::path::PathBuf;

use crate::{
    lexer::Lexer,
    parser::{item::Item, Parser},
};

#[derive(Debug, Clone, PartialEq)]
pub struct PreparsedSourceFile<'a> {
    pub path: PathBuf,
    pub src: &'a str,
    pub lexer: Lexer<'a>,
    pub parser: Parser<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourceFile<'a> {
    pub preparsed: PreparsedSourceFile<'a>,
    pub ast: Vec<Item<'a>>,
}
